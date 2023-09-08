use chrono::NaiveDateTime;
use poem_openapi::{
    ApiResponse,
    Object,
};
use proc_macros::FieldEnum;

use crate::domain::models;

#[derive(ApiResponse, Debug)]
pub enum Create {
    #[oai(status = 201)]
    Ok,
}

#[derive(Debug, Object, FieldEnum)]
#[oai(rename = "GetUserResponse")]
pub struct Get {
    /// Name
    pub name: String,
    /// Full name
    pub full_name: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Created at
    pub created_at: NaiveDateTime,
    /// Deleted at
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<models::User> for Get {
    fn from(user: models::User) -> Self {
        Self {
            name: user.name,
            full_name: user.full_name,
            email: Some(user.email),
            created_at: user.created_at,
            deleted_at: user.deleted_at,
        }
    }
}

#[derive(ApiResponse, Debug)]
pub enum Update {
    #[oai(status = 200)]
    Ok,
}

#[derive(Debug, Object)]
#[oai(rename = "SearchUserResponse")]
pub struct Search {
    /// Users
    pub users: Vec<Get>,
    /// Total users
    pub total_users: i64,
}
