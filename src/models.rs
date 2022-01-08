use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub code: i64,
    pub zone: String,
    pub kind: String,
    pub max_speed: i64,
    pub vendor: String,
    pub helmet: String,
    pub container: bool,
    pub license_plate: String,
}

#[derive(Serialize, Deserialize)]
pub struct VehicleStatus {
    pub vehicle_id: String,
    pub lat: f64,
    pub lng: f64,
    pub battery: i64,
    pub state: String,
    pub rentable: bool,
    pub timestamp: String,
}
