use std::sync::Arc;

use sqlx::PgPool;

use crate::Config;

mod database;
pub mod kafka;
mod prometheus;
mod redis;

pub use prometheus::{
    prometheus_metrics,
    PrometheusMetrics,
};
pub use redis::new_redis_storage;

#[derive(Clone, Debug)]
pub struct Infrastructure {
    pub db_pool: Arc<PgPool>,
    pub kafka: Arc<kafka::Kafka>,
}

impl Infrastructure {
    pub async fn new(config: &Config) -> Self {
        Self {
            db_pool: Arc::new(database::new_db_pool(config).await),
            kafka: Arc::new(kafka::Kafka::new(config)),
        }
    }
}
