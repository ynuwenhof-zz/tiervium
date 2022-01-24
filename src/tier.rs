use crate::models::{Attr, Log, Vehicle};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use reqwest::{Client, Url};
use serde_json::Value;

pub async fn zones(http: &Client) -> anyhow::Result<Vec<String>> {
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

pub async fn vehicles(
    http: &Client,
    zone: impl AsRef<str>,
) -> anyhow::Result<(Vec<Vehicle>, Vec<Log>)> {
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
            uuid: id.to_string(),
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
            vehicle_uuid: id.to_string(),
            time: attr.last_location_update.parse::<DateTime<Utc>>()?,
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
pub async fn vehicle(http: &Client, uuid: impl AsRef<str>) -> anyhow::Result<(Vehicle, Log)> {
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
        uuid: id.to_string(),
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
        vehicle_uuid: id.to_string(),
        time: attr.last_location_update.parse::<DateTime<Utc>>()?,
        lat: attr.lat,
        lng: attr.lng,
        battery: attr.battery_level,
        rentable: attr.is_rentable,
        state: attr.state,
    };

    Ok((vehicle, log))
}
