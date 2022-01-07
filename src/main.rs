mod models;

use anyhow::Result;
use serde::Deserialize;

use reqwest::header::HeaderMap;

use tokio::fs;

#[derive(Deserialize)]
struct Config {
    mongodb: String,
    key: String,
    zones: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("Config.toml").await?;
    let config: Config = toml::from_str(&content)?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", config.key.parse()?);

    let http_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(())
}
