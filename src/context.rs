use crate::cache::Cache;
use std::sync::Arc;

pub struct Context {
    pub http: Arc<reqwest::Client>,
    pub mongo: Arc<mongodb::Client>,
    pub cache: Arc<Cache>,
}

impl Context {
    pub fn new(http: Arc<reqwest::Client>, mongo: Arc<mongodb::Client>, cache: Arc<Cache>) -> Self {
        Self { http, mongo, cache }
    }
}
