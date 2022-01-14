use mongodb::bson;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub uuid: Uuid,
    pub code: i32,
    pub max_speed: i32,
    pub has_box: bool,
    pub has_helmet: bool,
    pub zone: String,
    pub kind: String,
    pub vendor: String,
    pub license_plate: String,
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub vehicle_uuid: Uuid,
    pub time: bson::DateTime,
    pub lat: f32,
    pub lng: f32,
    pub battery: i32,
    pub rentable: bool,
    pub state: String,
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
