use crate::{
    application::dtos,
    domain::{repositories, models},
    infrastructure::Infrastructure,
};

mod api_key;
pub use api_key::ApiKey;

pub struct User {
    repository: repositories::User,
}

impl User {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        let repository = repositories::User::new(infrastructure);
        Self { repository }
    }

    pub async fn create(
        &self,
        payload: dtos::user::requests::Create,
    ) -> Result<i32, dtos::Error> {
        let password_hash = super::Auth::get_secret_hash(&payload.password)?;

        let id = self.repository.create(payload, password_hash).await?;

        Ok(id)
    }

    pub async fn get(
        &self,
        id: &i32,
    ) -> Result<dtos::user::responses::Get, dtos::Error> {
        let user = self
            .repository
            .get(id)
            .await
            .map(dtos::user::responses::Get::from)?;

        Ok(user)
    }

    pub async fn get_by_name(
        &self,
        name: &str,
    ) -> Result<dtos::user::responses::Get, dtos::Error> {
        let user = self
            .repository
            .get_by_name(name)
            .await
            .map(dtos::user::responses::Get::from)?;

        Ok(user)
    }

    pub async fn update(
        &self,
        name: String,
        payload: dtos::user::requests::Update,
    ) -> Result<(), dtos::Error> {
        let password_hash = payload
            .password
            .as_ref()
            .map(|password| super::Auth::get_secret_hash(password))
            .transpose()?;

        self.repository.update(name, payload, password_hash).await?;

        Ok(())
    }

    pub async fn delete(
        &self,
        name: String,
    ) -> Result<(), dtos::Error> {
        self.repository.delete(name).await?;

        Ok(())
    }

    pub async fn search(
        &self,
        query: dtos::user::requests::Search,
    ) -> Result<dtos::user::responses::Search, dtos::Error> {
        let (users, total_users) = self.repository.search(query).await?;

        let result = dtos::user::responses::Search {
            users: users
                .into_iter()
                .map(dtos::user::responses::Get::from)
                .collect(),
            total_users,
        };

        Ok(result)
    }
}
