mod actions;
mod cache;
mod context;
mod models;
mod tier;

use anyhow::Result;
use cache::Cache;
use context::Context;
use futures::future;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use sqlx::MySqlPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::{fs, time};

#[derive(Deserialize)]
struct Config {
    delay: u64,
    timeout: u64,
    mysql: String,
    key: String,
    zones: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("Config.toml").await?;
    let config: Config = toml::from_str(&content)?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", config.key.parse()?);

    let http = Arc::new(
        Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .default_headers(headers)
            .build()?,
    );

    let zones = match config.zones {
        Some(zones) => zones,
        None => tier::zones(&http).await?,
    };

    let pool = Arc::new(MySqlPool::connect(&config.mysql).await?);
    let cache = Arc::new(Cache::new());
    let ctx = Context::new(http, pool, cache);

    loop {
        let mut handles = Vec::with_capacity(zones.len());

        for zone in &zones {
            let ctx = ctx.clone();
            let zone = zone.clone();

            let handle = tokio::spawn(async move {
                if let Err(err) = handle(ctx, zone.clone()).await {
                    println!("{}: {}", zone, err);
                }
            });

            handles.push(handle);
        }

        future::join_all(handles).await;
        time::sleep(Duration::from_secs(config.delay)).await;
    }
}

async fn handle(ctx: Context, zone: impl AsRef<str>) -> anyhow::Result<()> {
    let (vehicles, mut logs) = tier::vehicles(&ctx.http, zone.as_ref()).await?;

    actions::add_vehicles(&ctx.pool, &vehicles).await?;

    let mut new_logs = Vec::new();
    let mut new_cache = logs.clone();

    let cached_logs = ctx.cache.logs(zone.as_ref()).unwrap_or_default();
    for cached_log in &cached_logs {
        let log = match logs
            .iter()
            .position(|l| l.vehicle_uuid == cached_log.vehicle_uuid)
        {
            Some(i) => logs.swap_remove(i),
            None => {
                let hidden_log = tier::vehicle(&ctx.http, &cached_log.vehicle_uuid).await?.1;

                new_cache.push(hidden_log.clone());
                hidden_log
            }
        };

        if cached_log.time != log.time && cached_log.distance(&log) > 10.0 {
            new_logs.push(log);
        }
    }

    for log in logs.drain(..) {
        if let Some(existing_log) = actions::latest_log(&ctx.pool, &log.vehicle_uuid).await? {
            if log.time == existing_log.time || log.distance(&existing_log) < 10.0 {
                continue;
            }
        }

        new_logs.push(log);
    }

    actions::add_logs(&ctx.pool, &new_logs).await?;
    ctx.cache.update_logs(zone.as_ref().to_owned(), new_cache);
    Ok(())
}
