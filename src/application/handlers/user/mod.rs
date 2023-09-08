use poem::{
    session::Session,
    web::Data,
};
use poem_openapi::{
    param::Path,
    payload::Json,
    OpenApi,
};

use crate::{
    application::{
        dtos,
        Tags,
    },
    domain::services,
    infrastructure::Infrastructure,
};

pub mod api_key;

#[derive(Debug)]
pub struct Api;

#[OpenApi]
impl Api {
    /// Create user
    #[oai(path = "/users", method = "post", tag = "Tags::User")]
    async fn create(
        &self,
        Json(payload): Json<dtos::user::requests::Create>,
        session: &Session,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<dtos::user::responses::Create, dtos::Error> {
        let service = services::User::new(infrastructure);

        let id = service
            .create(payload)
            .await?;

        session.set("id", id);

        Ok(dtos::user::responses::Create::Ok)
    }

    /// Get user
    #[oai(path = "/users/:name", method = "get", tag = "Tags::User")]
    async fn get(
        &self,
        Path(name): Path<String>,
        session: &Session,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<Json<dtos::user::responses::Get>, dtos::Error> {
        let service = services::User::new(infrastructure);

        let mut user = service.get_by_name(&name).await?;

        let id = session
            .get::<i32>("id")
            .ok_or(dtos::Error::Unauthorized(Json(
                "unauthorized".to_string().into(),
            )))?;
        let current_user = service.get(&id).await;

        if let Ok(current_user) = current_user {
            if current_user.name != name {
                user.email = None;
            }
        }

        Ok(Json(user))
    }

    /// Update user
    #[oai(path = "/users/:name", method = "patch", tag = "Tags::User")]
    async fn update(
        &self,
        Path(name): Path<String>,
        Json(payload): Json<dtos::user::requests::Update>,
        session: &Session,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<dtos::user::responses::Update, dtos::Error> {
        let id = session
            .get::<i32>("id")
            .ok_or(dtos::Error::Unauthorized(Json(
                "unauthorized".to_string().into(),
            )))?;

        let service = services::User::new(infrastructure);

        let user = service.get(&id).await.map_err(|_| {
            dtos::Error::Unauthorized(Json("unauthorized".to_string().into()))
        })?;

        if user.name != name {
            return Err(dtos::Error::Forbidden(Json("forbidden".to_string().into())));
        }

        service
            .update(name, payload)
            .await
            .map(|_| dtos::user::responses::Update::Ok)
    }

    /// Search user
    #[oai(path = "/users", method = "get", tag = "Tags::User")]
    async fn search(
        &self,
        query: dtos::user::requests::Search,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<Json<dtos::user::responses::Search>, dtos::Error> {
        let service = services::User::new(infrastructure);
        service.search(query).await.map(Json)
    }

    /// Delete user
    #[oai(path = "/users/:name", method = "delete", tag = "Tags::User")]
    async fn delete(
        &self,
        Path(name): Path<String>,
        session: &Session,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<(), dtos::Error> {
        let id = session
            .get::<i32>("id")
            .ok_or(dtos::Error::Unauthorized(Json(
                "unauthorized".to_string().into(),
            )))?;

        let service = services::User::new(infrastructure);

        let user = service.get(&id).await?;

        if user.name != name {
            return Err(dtos::Error::Forbidden(Json("forbidden".to_string().into())));
        }

        service.delete(name).await?;

        Ok(())
    }
}
