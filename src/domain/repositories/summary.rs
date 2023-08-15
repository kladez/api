use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    application::dtos,
    domain::errors::Error,
    infrastructure::{
        kafka::{
            self,
            Kafka,
        },
        Infrastructure,
    },
};
pub struct Summary {
    db_pool: Arc<PgPool>,
    kafka: Arc<Kafka>,
    consumer: Option<kafka::Consumer>,
}

impl Summary {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        Self {
            db_pool: Arc::clone(&infrastructure.db_pool),
            kafka: Arc::clone(&infrastructure.kafka),
            consumer: None,
        }
    }

    pub async fn get(&mut self) -> Result<dtos::summary::Summary, Error> {
        match &self.consumer {
            None => {
                self.consumer = Some(
                    self.kafka
                        .get_consumer(&[kafka::Topic::UserRegistrations])?,
                )
            }
            Some(consumer) => {
                consumer.recv().await?;
            }
        };

        let users_count = sqlx::query!("SELECT COUNT(*) as count FROM users")
            .fetch_one(&*self.db_pool)
            .await?
            .count
            .unwrap_or(0);

        Ok(dtos::summary::Summary { users_count })
    }
}
