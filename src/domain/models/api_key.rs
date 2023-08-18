use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct ApiKey {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub valid_until: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub user_id: i32,
}
