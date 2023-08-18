use poem::web::Data;
use poem_openapi::{
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
    /// Create a new user
    #[oai(path = "/users", method = "post", tag = "Tags::User")]
    async fn create(
        &self,
        Data(infrastructure): Data<&Infrastructure>,
        Json(payload): Json<dtos::user::requests::Create>,
    ) -> Result<dtos::user::responses::Create, dtos::Error> {
        let service = services::User::new(infrastructure);
        service
            .create(payload)
            .await
            .map(|_| dtos::user::responses::Create::Ok)
    }

    /// Get all users
    #[oai(path = "/users", method = "get", tag = "Tags::User")]
    async fn get_all(
        &self,
        Data(infrastructure): Data<&Infrastructure>,
    ) -> Result<Json<Vec<dtos::user::responses::Get>>, dtos::Error> {
        let service = services::User::new(infrastructure);
        service.get_all().await.map(Json)
    }
}
