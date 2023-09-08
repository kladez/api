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
        payload: dtos::user::requests::Create,
        password_hash: String,
    ) -> Result<i32, Error> {
        let id = sqlx::query!(
            "INSERT INTO users (
                name,
                full_name,
                email,
                password_hash
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            )
            RETURNING id",
            payload.name,
            payload.full_name,
            payload.email.to_string(),
            password_hash,
        )
        .fetch_one(&*self.db_pool)
        .await?
        .id;

        self.kafka
            .send(kafka::Topic::UserRegistrations, &[0], &())
            .await?;

        Ok(id)
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

    pub async fn update(
        &self,
        name: String,
        payload: dtos::user::requests::Update,
        password_hash: Option<String>,
    ) -> Result<(), Error> {
        let mut query = String::from("UPDATE users SET ");
        let mut params: Vec<String> = Vec::new();

        fn add_param(
            field_name: &str,
            field: Option<String>,
            query: &mut String,
            params: &mut Vec<String>,
        ) {
            if let Some(value) = field {
                if !params.is_empty() {
                    query.push_str(", ");
                }
                query.push_str(&format!("{} = ${}", field_name, params.len() + 1));
                params.push(value);
            }
        }

        add_param("name", payload.name, &mut query, &mut params);
        add_param("full_name", payload.full_name, &mut query, &mut params);
        add_param(
            "email",
            payload.email.map(|e| e.to_string()),
            &mut query,
            &mut params,
        );
        add_param("password_hash", password_hash, &mut query, &mut params);

        if params.is_empty() {
            return Ok(());
        }

        query.push_str(&format!(" WHERE name = ${}", params.len() + 1));
        params.push(name);

        let mut db_query = sqlx::query(&query);
        for param in &params {
            db_query = db_query.bind(param.clone());
        }
        db_query.execute(&*self.db_pool).await?;

        Ok(())
    }

    pub async fn search(
        &self,
        query: dtos::user::requests::Search,
    ) -> Result<(Vec<models::User>, i64), Error> {
        let search_query = match &query.query {
            Some(q) => format!("%{}%", q),
            None => "%".to_string(),
        };

        let sort_by = query.sort_by.map(|field| field.to_string());
        let page = query.page.unwrap_or(0);
        let page_size = query.page_size.unwrap_or(2);

        let users = sqlx::query_as!(
            models::User,
            "SELECT * FROM users
            WHERE name ILIKE $1
            ORDER BY $2
            OFFSET $3
            LIMIT $4",
            search_query,
            sort_by,
            page * page_size,
            page_size,
        )
        .fetch_all(&*self.db_pool)
        .await?;

        let total_users = sqlx::query!(
            "SELECT COUNT(*) AS count FROM users
            WHERE name ILIKE $1",
            search_query,
        )
        .fetch_one(&*self.db_pool)
        .await?
        .count
        .unwrap_or(0);

        Ok((users, total_users))
    }

    pub async fn delete(
        &self,
        name: String,
    ) -> Result<(), Error> {
        sqlx::query!("UPDATE users SET deleted_at = NOW() WHERE name = $1", name)
            .execute(&*self.db_pool)
            .await?;

        Ok(())
    }
}
