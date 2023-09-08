use poem_openapi::{ApiResponse, Object};

#[derive(ApiResponse, Debug)]
pub enum Auth {
    #[oai(status = 200)]
    Ok,
}

#[derive(Debug, Object)]
pub struct Check {
    pub name: String,
}
