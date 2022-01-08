mod models;
mod tier;

use tokio::fs;
use serde::Deserialize;
use anyhow::{Result, Context};
use reqwest::header::HeaderMap;

#[derive(Deserialize)]
struct Config {
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

    let zones = match config.zones {
        Some(zones) => zones,
        None => tier::zones(&http_client).await
            .with_context(|| "Failed to retrieve tier zones")?,
    };

    Ok(())
}
