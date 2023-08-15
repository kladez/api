use chrono::NaiveDateTime;
use poem_openapi::Object;

#[derive(Debug, Object)]
#[oai(rename = "CreateApiKeyRequest")]
pub struct Create {
    /// Name
    #[oai(validator(max_length = 64))]
    pub name: String,
    /// Valid until
    pub valid_until: NaiveDateTime,
}
