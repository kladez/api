use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    application::dtos,
    domain::{
        errors::Error,
        models,
    },
    infrastructure::Infrastructure,
};

pub struct ApiKey {
    db_pool: Arc<PgPool>,
}

impl ApiKey {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        Self {
            db_pool: Arc::clone(&infrastructure.db_pool),
        }
    }

    pub async fn create(
        &self,
        payload: &dtos::user::api_key::requests::Create,
        key: &str,
        user_id: &i32,
    ) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO api_keys (
                name,
                valid_until,
                key,
                user_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            )",
            payload.name,
            payload.valid_until,
            key,
            user_id,
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get(
        &self,
        user_id: &i32,
    ) -> Result<Vec<models::ApiKey>, Error> {
        let api_keys = sqlx::query_as!(
            models::ApiKey,
            "SELECT * FROM api_keys WHERE user_id = $1",
            user_id,
        )
        .fetch_all(&*self.db_pool)
        .await?;

        Ok(api_keys)
    }
}
