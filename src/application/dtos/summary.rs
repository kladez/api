use poem_openapi::Object;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Object, Deserialize, Serialize)]
#[oai(rename = "Summary")]
pub struct Summary {
    pub users_count: i64,
}
