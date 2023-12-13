use crate::{
    domain::models,
    presentation::dtos::{
        self,
        pagination::Pagination,
        user::OrderBy,
    },
};

#[derive(Debug)]
pub struct User {
    pool: sqlx::PgPool,
}

impl User {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user: &dtos::user::CreateRequest,
        password_hash: &String,
    ) -> Result<(), sqlx::Error> {
        let existing_user = sqlx::query_as!(
            models::User,
            r#"
                SELECT *
                FROM users
                WHERE name = $1
                OR email = $2
            "#,
            &user.name,
            &user.email,
        )
        .fetch_one(&self.pool)
        .await;

        if existing_user.is_ok() {
            return Err(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "User already exists",
            )));
        }

        sqlx::query!(
            r#"
                INSERT INTO users (
                    name,
                    email,
                    password_hash
                ) VALUES (
                    $1,
                    $2,
                    $3
                )
            "#,
            &user.name,
            &user.email,
            password_hash
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }

    pub async fn get_all(
        &self,
        pagination: &Pagination<OrderBy>,
    ) -> Result<Vec<models::User>, sqlx::Error> {
        let offset = (pagination.page - 1) * pagination.size;

        let query_str = format!(
            r#"
                SELECT *
                FROM users
                ORDER BY {} {}
                LIMIT $1
                OFFSET $2
            "#,
            pagination.order_by, pagination.order,
        );

        sqlx::query_as::<_, models::User>(&query_str)
            .bind(pagination.size)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get(
        &self,
        id: &i32,
    ) -> Result<models::User, sqlx::Error> {
        sqlx::query_as!(
            models::User,
            r#"
                SELECT *
                FROM users
                WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: &i32,
        user: &dtos::user::UpdateRequest,
        password_hash: &Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                UPDATE users
                SET
                    name = COALESCE($1, name),
                    email = COALESCE($2, email),
                    password_hash = COALESCE($3, password_hash)
                WHERE id = $4
            "#,
            user.name.as_ref(),
            user.email.as_ref(),
            password_hash.as_ref(),
            id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }

    pub async fn delete(
        &self,
        id: &i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
    }
}
