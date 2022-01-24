use crate::cache::Cache;
use reqwest::Client;
use sqlx::MySqlPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub http: Arc<Client>,
    pub pool: Arc<MySqlPool>,
    pub cache: Arc<Cache>,
}

impl Context {
    pub fn new(http: Arc<Client>, pool: Arc<MySqlPool>, cache: Arc<Cache>) -> Self {
        Self { http, pool, cache }
    }
}
