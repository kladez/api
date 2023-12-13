use crate::{
    domain::repositories,
    presentation::dtos::{
        self,
        pagination::Pagination,
        user::OrderBy,
    },
};

#[derive(Debug)]
pub struct User {
    repository: repositories::User,
}

impl User {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repository = repositories::User::new(pool);
        Self { repository }
    }

    pub async fn create(
        &self,
        user: &dtos::user::CreateRequest,
    ) -> Result<(), String> {
        let password_hash = String::new();
        self.repository
            .create(user, &password_hash)
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn get_all(
        &self,
        pagination: &Pagination<OrderBy>,
    ) -> Result<Vec<dtos::user::GetResponse>, String> {
        self.repository
            .get_all(pagination)
            .await
            .map(|users| users.into_iter().map(Into::into).collect())
            .map_err(|err| err.to_string())
    }

    pub async fn get(
        &self,
        id: &i32,
    ) -> Result<dtos::user::GetResponse, String> {
        self.repository
            .get(id)
            .await
            .map(Into::into)
            .map_err(|err| err.to_string())
    }

    pub async fn update(
        &self,
        id: &i32,
        user: &dtos::user::UpdateRequest,
    ) -> Result<(), String> {
        let password_hash = Some(String::new());

        self.repository
            .update(id, user, &password_hash)
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn delete(
        &self,
        id: &i32,
    ) -> Result<(), String> {
        self.repository.delete(id).await.map_err(|err| err.to_string())
    }
}
