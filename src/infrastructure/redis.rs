use redis::{
    aio::ConnectionManager,
    Client,
};

use crate::Config;

pub async fn new_redis_storage(config: &Config) -> ConnectionManager {
    let client = Client::open(config.redis_url.as_str()).unwrap();
    ConnectionManager::new(client).await.unwrap()
}
