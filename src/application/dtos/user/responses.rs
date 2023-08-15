use poem_openapi::{Object, ApiResponse};

use crate::domain::models;

#[derive(ApiResponse, Debug)]
pub enum Create {
    #[oai(status = 201)]
    Ok,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
#[oai(rename = "GetUserResponse")]
pub struct Get {
    name: String,
}

impl From<models::User> for Get {
    fn from(user: models::User) -> Self {
        Self { name: user.name }
    }
}
