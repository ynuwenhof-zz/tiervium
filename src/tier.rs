use serde_json::Value;
use anyhow::{anyhow, Result};

use crate::models::{VehicleLog, VehicleAttributes};

pub async fn get_zones(http_client: &reqwest::Client) -> Result<Vec<String>> {
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

pub async fn get_vehicles_by_zone(http_client: &reqwest::Client, zone: &str) -> Result<Vec<VehicleLog>> {
    let content = http_client
        .get(format!("https://platform.tier-services.io/v2/vehicle?zoneId={}", zone))
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&content)?;
    let data = json["data"]
        .as_array()
        .ok_or(anyhow!(""))?; // TODO: Proper error response

    let mut logs = Vec::with_capacity(data.len());

    for entry in data {
        let id = entry["id"]
            .as_str()
            .ok_or(anyhow!(""))? // TODO: Proper error response
            .to_string();

        let attr: VehicleAttributes = serde_json::from_value(entry["attributes"].to_owned())?;
        let log = VehicleLog::from((id, attr));
        logs.push(log);
    }

    Ok(logs)
}

pub async fn get_vehicle(http_client: &reqwest::Client, uuid: &str) -> Result<VehicleLog> {
    let content = http_client
        .get(format!("https://platform.tier-services.io/v1/vehicle/{}", uuid))
        .send()
        .await?
        .text()
        .await?;
    
    let json: Value = serde_json::from_str(&content)?;
    let id = json["data"]["id"]
        .as_str()
        .ok_or(anyhow!(""))? // TODO: Proper error response
        .to_string();

    let attr: VehicleAttributes = serde_json::from_value(json["data"]["attributes"].to_owned())?;
    Ok(VehicleLog::from((id, attr)))
}
