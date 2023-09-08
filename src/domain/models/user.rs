use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub full_name: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
