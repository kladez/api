use poem::{
    session::Session,
    web::Data,
    Request,
};
use poem_openapi::{
    auth::ApiKey,
    payload::Json,
    OpenApi,
    SecurityScheme,
};

use crate::{
    application::{
        dtos::{
            self,
            auth,
        },
        Tags,
    },
    domain::{
        models,
        services,
    },
    infrastructure::Infrastructure,
};

/// API key authorization
#[allow(missing_debug_implementations)]
#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_in = "header", key_name = "X-API-Key")]
pub struct ApiKeyAuth(pub poem_openapi::auth::ApiKey);

/// Cookie authorization
#[allow(missing_debug_implementations)]
#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_in = "cookie", key_name = "session")]
pub struct CookieAuth(pub poem_openapi::auth::ApiKey);

#[derive(Debug)]
pub struct Api;

#[OpenApi]
impl Api {
    /// Auth
    #[oai(path = "/auth", method = "post", tag = "Tags::Auth")]
    pub async fn create(
        &self,
        Json(payload): Json<dtos::auth::requests::Auth>,
        session: &Session,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<dtos::auth::responses::Auth, dtos::Error> {
        let service = services::Auth::new(infrastructure);

        let user = service.auth(&payload).await.map_err(|_| {
            dtos::Error::Unauthorized(Json("Authentication failed".to_string().into()))
        })?;

        session.set("id", user.id);

        Ok(dtos::auth::responses::Auth::Ok)
    }

    #[oai(path = "/auth/check", method = "get", tag = "Tags::Auth")]
    pub async fn read(
        &self,
        session: &Session,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<Json<dtos::auth::responses::Check>, dtos::Error> {
        let id = session
            .get::<i32>("id")
            .ok_or(dtos::Error::Unauthorized(Json(
                "unauthorized".to_string().into(),
            )))?;

        let service = services::User::new(infrastructure);

        let user = service.get(&id).await?;

        Ok(Json(dtos::auth::responses::Check {
            name: user.name,
        }))
    }

    #[oai(path = "/auth", method = "delete", tag = "Tags::Auth")]
    pub async fn delete(
        &self,
        session: &Session,
    ) -> Result<(), dtos::Error> {
        let id = session
            .get::<i32>("id")
            .ok_or(dtos::Error::Unauthorized(Json(
                "unauthorized".to_string().into(),
            )))?;

        session.purge();

        Ok(())
    }
}
