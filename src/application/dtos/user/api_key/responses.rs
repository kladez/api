use poem_openapi::{
    payload::Json,
    ApiResponse,
    Object,
};

use crate::domain::models;

#[derive(Debug, ApiResponse)]
pub enum Create {
    #[oai(status = 201)]
    Ok(Json<ApiKey>),
}

#[derive(Debug, Object)]
#[oai(rename = "CreateApiKeyResponse")]
pub struct ApiKey {
    pub api_key: String,
}

#[derive(Debug, Object)]
#[oai(rename = "GetApiKeyResponse")]
pub struct Get {
    pub name: String,
}

impl From<models::ApiKey> for Get {
    fn from(api_key: models::ApiKey) -> Self {
        Self { name: api_key.name }
    }
}
