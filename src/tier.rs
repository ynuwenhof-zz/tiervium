use serde_json::Value;
use anyhow::{anyhow, Result};

pub async fn zones(http_client: reqwest::Client) -> Result<Vec<String>> {
    let content = http_client
        .get("https://platform.tier-services.io/v1/zone?type=root")
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&content)?;
    let data = json["data"]
        .as_array()
        .ok_or(anyhow!(""))?; // TODO: Proper error response

    let mut zones = Vec::with_capacity(data.len());

    for entry in data {
        let zone = entry["id"]
            .as_str()
            .ok_or(anyhow!(""))? // TODO: Proper error response
            .to_string();

        zones.push(zone);
    }

    Ok(zones)
}