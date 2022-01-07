use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    id: String,
    code: i64,
    zone: String,
    kind: String,
    max_speed: i64,
    vendor: String,
    helmet: String,
    container: bool,
    license_plate: String,
}

#[derive(Serialize, Deserialize)]
pub struct VehicleStatus {
    vehicle_id: String,
    lat: f64,
    lng: f64,
    battery: i64,
    state: String,
    rentable: bool,
    timestamp: String,
}