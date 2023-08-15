use poem_openapi::Object;

#[derive(Debug, Object)]
#[oai(rename = "AuthRequest")]
pub struct Auth {
    pub name: String,
    pub password: String,
}
