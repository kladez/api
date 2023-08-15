use crate::{
    application::dtos,
    domain::repositories,
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
        user: dtos::user::requests::Create,
    ) -> Result<(), dtos::Error> {
        let password_hash = super::Auth::get_secret_hash(&user.password)?;

        self.repository.create(user, password_hash).await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<dtos::user::responses::Get>, dtos::Error> {
        let users = self.repository.get_all().await.map(|users| {
            users
                .into_iter()
                .map(dtos::user::responses::Get::from)
                .collect()
        })?;

        Ok(users)
    }

    pub async fn get(
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
}
