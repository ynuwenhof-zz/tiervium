use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub code: i64,
    pub zone: String,
    pub kind: String,
    pub max_speed: i64,
    pub vendor: String,
    pub helmet: bool,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleAttributes {
    pub battery_level: i64,
    pub code: i64,
    pub has_helmet: bool,
    pub has_helmet_box: bool,
    pub iot_vendor: String,
    pub is_rentable: bool,
    pub last_location_update: String,
    pub last_state_change: String,
    pub lat: f64,
    pub licence_plate: String,
    pub lng: f64,
    pub max_speed: i64,
    pub state: String,
    pub vehicle_type: String,
    pub zone_id: String,
}

pub struct VehicleLog {
    pub vehicle: Vehicle,
    pub status: VehicleStatus,
}

impl From<(String, VehicleAttributes)> for VehicleLog {
    fn from(val: (String, VehicleAttributes)) -> Self {
        let attr = val.1;

        let vehicle = Vehicle {
            id: val.0.clone(),
            code: attr.code,
            zone: attr.zone_id,
            kind: attr.vehicle_type,
            max_speed: attr.max_speed,
            vendor: attr.iot_vendor,
            helmet: attr.has_helmet,
            container: attr.has_helmet_box,
            license_plate: attr.licence_plate,
        };

        let status = VehicleStatus {
            vehicle_id: val.0,
            lat: attr.lat,
            lng: attr.lng,
            battery: attr.battery_level,
            state: attr.state,
            rentable: attr.is_rentable,
            timestamp: attr.last_location_update,
        };

        VehicleLog {
            vehicle,
            status,
        }
    }
}