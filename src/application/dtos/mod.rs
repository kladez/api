use poem_openapi::{
    payload::Json,
    ApiResponse,
    Object,
};

pub mod auth;
pub mod summary;
pub mod user;

#[derive(Debug, Object)]
pub struct ErrorMessage {
    message: String,
}

impl From<String> for ErrorMessage {
    fn from(message: String) -> Self {
        Self { message }
    }
}

#[derive(ApiResponse, Debug)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(Json<ErrorMessage>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorMessage>),
    #[oai(status = 404)]
    NotFound(Json<ErrorMessage>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorMessage>),
}
