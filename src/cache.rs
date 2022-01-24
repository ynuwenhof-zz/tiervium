use crate::models::Log;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default)]
pub struct Cache {
    logs: RwLock<HashMap<String, Vec<Log>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn logs(&self, zone: impl AsRef<str>) -> Option<Vec<Log>> {
        self.logs.read().unwrap().get(zone.as_ref()).cloned()
    }

    pub fn update_logs(&self, zone: String, logs: Vec<Log>) {
        self.logs.write().unwrap().insert(zone, logs);
    }
}
