use poem::{
    async_trait,
    FromRequest,
    Request,
    RequestBody,
};
use poem_openapi::{
    types::{
        Email,
        Password,
    },
    Object,
};
use serde::Deserialize;

use crate::application::dtos::SortOrder;

#[derive(Debug, Object)]
#[oai(rename = "CreateUserRequest")]
pub struct Create {
    /// Name
    pub name: String,
    /// Full name
    pub full_name: String,
    /// Email
    pub email: Email,
    /// Password
    pub password: Password,
}

#[derive(Debug, Object)]
#[oai(rename = "UpdateUserRequest")]
pub struct Update {
    /// Name
    pub name: Option<String>,
    /// Full name
    pub full_name: Option<String>,
    /// Email
    pub email: Option<Email>,
    /// Password
    pub password: Option<Password>,
}

#[derive(Debug, Object, Deserialize)]
#[oai(rename = "SearchUserRequest")]
pub struct Search {
    /// Query
    pub query: Option<String>,
    /// Page
    pub page: Option<i64>,
    /// Page size
    pub page_size: Option<i64>,
    /// Sort by
    pub sort_by: Option<super::responses::GetField>,
    /// Sort order
    pub sort_order: Option<SortOrder>,
}

#[async_trait]
impl<'a> FromRequest<'a> for Search {
    async fn from_request(
        req: &'a Request,
        _body: &mut RequestBody,
    ) -> poem::Result<Self> {
        Ok(req.params()?)
    }
}
