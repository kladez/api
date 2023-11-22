use serde::{
    Deserialize,
    Serialize,
};

use crate::domain::models;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    #[default]
    Id,
    Name,
    CreatedAt,
}

impl std::fmt::Display for OrderBy {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrderBy::Id => "id",
                OrderBy::Name => "name",
                OrderBy::CreatedAt => "created_at",
            }
        )
    }
}

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<models::User> for GetResponse {
    fn from(user: models::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
