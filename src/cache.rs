use crate::models::{Log, Vehicle};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct Cache {
    pub vehicles: RwLock<HashMap<String, Vec<Vehicle>>>,
    pub logs: RwLock<HashMap<String, Vec<Log>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn vehicles(&self, zone: impl AsRef<str>) -> Option<Vec<Vehicle>> {
        let vehicles = self.vehicles.read().await.get(zone.as_ref()).cloned();

        match vehicles {
            None => {
                {
                    self.vehicles
                        .write()
                        .await
                        .insert(zone.as_ref().to_string(), Vec::new());
                }

                self.vehicles.read().await.get(zone.as_ref()).cloned()
            }
            Some(vehicles) => Some(vehicles),
        }
    }

    pub async fn logs(&self, zone: impl AsRef<str>) -> Option<Vec<Log>> {
        let logs = self.logs.read().await.get(zone.as_ref()).cloned();

        match logs {
            None => {
                {
                    self.logs
                        .write()
                        .await
                        .insert(zone.as_ref().to_string(), Vec::new());
                }

                self.logs.read().await.get(zone.as_ref()).cloned()
            }
            Some(logs) => Some(logs),
        }
    }
}
