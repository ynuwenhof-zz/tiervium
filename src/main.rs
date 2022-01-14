mod cache;
mod context;
mod models;
mod tier;

use crate::cache::Cache;
use crate::context::Context;
use anyhow::{anyhow, Result};
use futures::future;
use models::{Log, Vehicle};
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::options::IndexOptions;
use mongodb::options::UpdateOptions;
use mongodb::IndexModel;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::time;

const DATABASE: &str = "tiervium";
const VEHICLE_COLLECTION: &str = "vehicles";
const LOG_COLLECTION: &str = "logs";

#[derive(Deserialize)]
struct Config {
    delay: u64,
    mongodb: String,
    key: String,
    zones: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("Config.toml").await?;
    let config: Config = toml::from_str(&content)?;

    let mongo = Arc::new(mongodb::Client::with_uri_str(&config.mongodb).await?);

    {
        let database = mongo.database(DATABASE);
        let index_options = IndexOptions::builder().unique(true).build();

        let index_model = IndexModel::builder()
            .keys(doc!("uuid": 1))
            .options(index_options)
            .build();

        database
            .collection::<Vehicle>(VEHICLE_COLLECTION)
            .create_index(index_model, None)
            .await?;

        let index_model = IndexModel::builder().keys(doc!("vehicle_uuid": 1)).build();

        database
            .collection::<Log>(LOG_COLLECTION)
            .create_index(index_model, None)
            .await?;
    }

    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", config.key.parse()?);

    let http = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let http = Arc::new(http);

    let zones = match config.zones {
        Some(zones) => zones,
        None => tier::get_zones(http.clone()).await?,
    };

    let cache = Arc::new(Cache::new());

    loop {
        let mut handles = Vec::with_capacity(zones.len());

        for zone in &zones {
            let ctx = Context::new(http.clone(), mongo.clone(), cache.clone());
            let zone = zone.clone();

            let handle = tokio::spawn(async move { handle(ctx, zone).await });
            handles.push(handle);
        }

        future::join_all(handles).await;
        time::sleep(Duration::from_secs(config.delay)).await;
    }
}

async fn handle(ctx: Context, zone: impl AsRef<str>) -> Result<()> {
    let (vehicles, logs) = tier::get_vehicles_by_zone(ctx.http, zone.as_ref()).await?;

    let cached_vehicles = ctx
        .cache
        .vehicles(zone.as_ref())
        .await
        .ok_or(anyhow!("expected vehicles in cache"))?;

    let cached_logs = ctx
        .cache
        .logs(zone.as_ref())
        .await
        .ok_or(anyhow!("expected logs in cache"))?;

    let database = ctx.mongo.database(DATABASE);

    let mut session = ctx.mongo.start_session(None).await?;
    session.start_transaction(None).await?;

    for vehicle in &vehicles {
        if cached_vehicles.contains(vehicle) {
            continue;
        }

        database
            .collection::<Vehicle>(VEHICLE_COLLECTION)
            .update_one_with_session(
                doc!("uuid": &vehicle.uuid),
                doc!("$setOnInsert": bson::to_bson(&vehicle)?.as_document()),
                UpdateOptions::builder().upsert(true).build(),
                &mut session,
            )
            .await?;
    }

    session.commit_transaction().await?;
    session.start_transaction(None).await?;

    let mut new_logs = Vec::new();
    let log_collection = database.collection::<Log>(LOG_COLLECTION);

    for log in &logs {
        if let Some(cached_log) = cached_logs
            .iter()
            .find(|l| l.vehicle_uuid == log.vehicle_uuid)
        {
            if cached_log.time == log.time || cached_log.lat == log.lat && cached_log.lng == log.lng
            {
                continue;
            }
        }

        if let Some(existing_log) = log_collection
            .find_one_with_session(
                doc!("vehicle_uuid": &log.vehicle_uuid),
                FindOneOptions::builder().sort(doc!("time": -1)).build(),
                &mut session,
            )
            .await?
        {
            if existing_log.time == log.time
                || existing_log.lat == log.lat && existing_log.lng == log.lng
            {
                continue;
            }
        }

        new_logs.push(log);
    }

    if !new_logs.is_empty() {
        log_collection
            .insert_many_with_session(new_logs, None, &mut session)
            .await?;
    }

    session.commit_transaction().await?;

    {
        ctx.cache
            .vehicles
            .write()
            .await
            .insert(zone.as_ref().to_string(), vehicles);

        ctx.cache
            .logs
            .write()
            .await
            .insert(zone.as_ref().to_string(), logs);
    }

    // TODO: Track rented vehicles

    Ok(())
}
