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
    pub async fn auth(
        &self,
        Data(infrastructure): Data<&Infrastructure>,
        Json(payload): Json<dtos::auth::requests::Auth>,
        session: &Session,
    ) -> Result<dtos::auth::responses::Auth, dtos::Error> {
        let service = services::Auth::new(infrastructure);

        let user = service.auth(&payload).await?;

        session.set("user_id", user.id);

        Ok(dtos::auth::responses::Auth::Ok)
    }
}
