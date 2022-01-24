use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::FromRow;

pub struct Vehicle {
    pub uuid: String,
    pub code: i32,
    pub max_speed: i32,
    pub has_box: bool,
    pub has_helmet: bool,
    pub zone: String,
    pub kind: String,
    pub vendor: String,
    pub license_plate: String,
}

#[derive(Clone, FromRow)]
pub struct Log {
    pub vehicle_uuid: String,
    pub time: DateTime<Utc>,
    pub lat: f32,
    pub lng: f32,
    pub battery: i32,
    pub rentable: bool,
    pub state: String,
}

impl Log {
    pub fn distance(&self, other: &Log) -> f32 {
        let lat = self.lat.to_radians();
        let lng = (self.lng - other.lng).to_radians();

        let other_lat = other.lat.to_radians();

        let x = lng.cos() * lat.cos() - other_lat.cos();
        let y = lng.sin() * lat.cos();
        let z = lat.sin() - other_lat.sin();

        ((x * x + y * y + z * z).sqrt() / 2.0).asin() * 2.0 * 6371e3
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attr {
    pub battery_level: i32,
    pub code: i32,
    pub has_helmet: bool,
    pub has_helmet_box: bool,
    pub iot_vendor: String,
    pub is_rentable: bool,
    pub last_location_update: String,
    pub last_state_change: String,
    pub lat: f32,
    pub licence_plate: String,
    pub lng: f32,
    pub max_speed: i32,
    pub state: String,
    pub vehicle_type: String,
    pub zone_id: String,
}
