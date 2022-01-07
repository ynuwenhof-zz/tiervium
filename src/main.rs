use anyhow::Result;
use serde::Deserialize;

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

    Ok(())
}
