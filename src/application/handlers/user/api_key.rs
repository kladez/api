use poem::{
    session::Session,
    web::Data,
};
use poem_openapi::{
    payload::Json,
    OpenApi,
};

use crate::{
    application::{
        dtos,
        handlers::auth::{
            ApiKeyAuth,
            CookieAuth,
        },
        Tags,
    },
    domain::services,
    infrastructure::Infrastructure,
};

#[derive(Debug)]
pub struct Api;

#[OpenApi]
impl Api {
    /// Create a new API key
    #[oai(path = "/users/api-keys", method = "post", tag = "Tags::ApiKey")]
    async fn create(
        &self,
        Data(infrastructure): Data<&Infrastructure>,
        Json(payload): Json<dtos::user::api_key::requests::Create>,
        session: &Session,
        // https://github.com/poem-web/poem/issues/626
        // _api_key_auth: ApiKeyAuth,
        _cookie_auth: CookieAuth,
    ) -> Result<dtos::user::api_key::responses::Create, dtos::Error> {
        let name = session
            .get::<i32>("name")
            .ok_or(dtos::Error::Unauthorized(Json(
                "unauthorized".to_string().into(),
            )))?;

        let service = services::user::ApiKey::new(infrastructure);
        service.create(&payload, &name).await.map(|api_key| {
            dtos::user::api_key::responses::Create::Ok(Json(
                dtos::user::api_key::responses::ApiKey { api_key },
            ))
        })
    }
}
