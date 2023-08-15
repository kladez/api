use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    application::dtos,
    domain::{
        errors::Error,
        models,
    },
    infrastructure::{
        kafka::{
            self,
            Kafka,
        },
        Infrastructure,
    },
};

mod api_key;
pub use api_key::ApiKey;

pub struct User {
    db_pool: Arc<PgPool>,
    kafka: Arc<Kafka>,
}

impl User {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        Self {
            db_pool: Arc::clone(&infrastructure.db_pool),
            kafka: Arc::clone(&infrastructure.kafka),
        }
    }

    pub async fn create(
        &self,
        user: dtos::user::requests::Create,
        password_hash: String,
    ) -> Result<(), Error> {
        sqlx::query_as!(
            models::User,
            "INSERT INTO users (
                    name,
                    email,
                    password_hash
                ) VALUES (
                    $1,
                    $2,
                    $3
                )
            ",
            user.name,
            user.email.to_string(),
            password_hash,
        )
        .execute(&*self.db_pool)
        .await?;

        self.kafka
            .send(kafka::Topic::UserRegistrations, &[0], &())
            .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<models::User>, Error> {
        let users = sqlx::query_as!(models::User, "SELECT * FROM users")
            .fetch_all(&*self.db_pool)
            .await?;

        Ok(users)
    }

    pub async fn get(
        &self,
        id: &i32,
    ) -> Result<models::User, Error> {
        let user = sqlx::query_as!(models::User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*self.db_pool)
            .await?;

        Ok(user)
    }

    pub async fn get_by_name(
        &self,
        name: &str,
    ) -> Result<models::User, Error> {
        let user = sqlx::query_as!(models::User, "SELECT * FROM users WHERE name = $1", name)
            .fetch_one(&*self.db_pool)
            .await?;

        Ok(user)
    }
}
