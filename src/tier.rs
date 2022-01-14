use crate::models::{Attr, Log, Vehicle};
use anyhow::{anyhow, Result};
use chrono::Utc;
use mongodb::bson;
use mongodb::bson::Uuid;
use reqwest::{Client, Url};
use serde_json::Value;
use std::sync::Arc;

pub async fn get_zones(http: Arc<Client>) -> Result<Vec<String>> {
    let content = http
        .get("https://platform.tier-services.io/v1/zone?type=root")
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&content)?;
    let data = json["data"]
        .as_array()
        .ok_or(anyhow!("invalid json, expected data array"))?;

    let mut zones = Vec::with_capacity(data.len());

    for item in data {
        let zone = item["id"]
            .as_str()
            .ok_or(anyhow!("invalid json, expected zone string"))?;

        zones.push(zone.to_string());
    }

    Ok(zones)
}

pub async fn get_vehicles_by_zone(
    http: Arc<Client>,
    zone: impl AsRef<str>,
) -> Result<(Vec<Vehicle>, Vec<Log>)> {
    let content = http
        .get(Url::parse_with_params(
            "https://platform.tier-services.io/v2/vehicle",
            &[("zoneId", zone)],
        )?)
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&content)?;
    let data = json["data"]
        .as_array()
        .ok_or(anyhow!("invalid json, expected data array"))?;

    let mut vehicles = Vec::with_capacity(data.len());
    let mut logs = Vec::with_capacity(data.len());

    for item in data {
        let id = item["id"]
            .as_str()
            .ok_or(anyhow!("invalid json, expected vehicle id string"))?;

        let attr: Attr = serde_json::from_value(item["attributes"].clone())?;

        let vehicle = Vehicle {
            uuid: Uuid::parse_str(id)?,
            code: attr.code,
            max_speed: attr.max_speed,
            has_box: attr.has_helmet_box,
            has_helmet: attr.has_helmet,
            zone: attr.zone_id,
            kind: attr.vehicle_type,
            vendor: attr.iot_vendor,
            license_plate: attr.licence_plate,
        };

        vehicles.push(vehicle);

        let log = Log {
            vehicle_uuid: Uuid::parse_str(id)?,
            time: bson::DateTime::from(attr.last_location_update.parse::<chrono::DateTime<Utc>>()?),
            lat: attr.lat,
            lng: attr.lng,
            battery: attr.battery_level,
            rentable: attr.is_rentable,
            state: attr.state,
        };

        logs.push(log);
    }

    Ok((vehicles, logs))
}

#[allow(dead_code)]
pub async fn get_vehicle_by_uuid(
    http: Arc<Client>,
    uuid: impl AsRef<str>,
) -> Result<(Vehicle, Log)> {
    let content = http
        .get(format!(
            "https://platform.tier-services.io/v1/vehicle/{}",
            uuid.as_ref()
        ))
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&content)?;
    let id = json["data"]["id"]
        .as_str()
        .ok_or(anyhow!("invalid json, expected vehicle id string"))?;

    let attr: Attr = serde_json::from_value(json["data"]["attributes"].clone())?;

    let vehicle = Vehicle {
        uuid: Uuid::parse_str(id)?,
        code: attr.code,
        max_speed: attr.max_speed,
        has_box: attr.has_helmet_box,
        has_helmet: attr.has_helmet,
        zone: attr.zone_id,
        kind: attr.vehicle_type,
        vendor: attr.iot_vendor,
        license_plate: attr.licence_plate,
    };

    let log = Log {
        vehicle_uuid: Uuid::parse_str(id)?,
        time: bson::DateTime::from(attr.last_location_update.parse::<chrono::DateTime<Utc>>()?),
        lat: attr.lat,
        lng: attr.lng,
        battery: attr.battery_level,
        rentable: attr.is_rentable,
        state: attr.state,
    };

    Ok((vehicle, log))
}
