mod models;
mod tier;

use tokio::fs;
use tokio::time;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use anyhow::{Result, Context};
use reqwest::header::HeaderMap;
use tier::get_vehicles_by_zone;

#[derive(Deserialize)]
struct Config {
    delay: u64,
    mongodb: String,
    key: String,
    zones: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("Config.toml")
        .await
        .with_context(|| "Failed to read contents from Config.toml")?;

    let config: Config = toml::from_str(&content)
        .with_context(|| "Failed to parse config")?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", config.key.parse()?);

    let http_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let http_client = Arc::new(http_client);

    let zones = match config.zones {
        Some(zones) => zones,
        None => tier::get_zones(&http_client).await
            .with_context(|| "Failed to retrieve tier zones")?,
    };

    loop {
        for zone in &zones {
            let http_client = http_client.clone();
            let zone = zone.clone();

            tokio::spawn(async move {
                handle(http_client, &zone).await
            });
        }

        time::sleep(Duration::from_secs(config.delay)).await;
    }
}

async fn handle(http_client: Arc<reqwest::Client>, zone: &str) -> Result<()> {
    let logs = get_vehicles_by_zone(http_client, zone).await?;
    // TODO: Process the logs

    Ok(())
}
