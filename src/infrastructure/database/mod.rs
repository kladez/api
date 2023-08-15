use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};

use crate::Config;

pub async fn new_db_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.db_url)
        .await
        .unwrap()
}
