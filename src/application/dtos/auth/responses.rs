use poem_openapi::ApiResponse;

#[derive(ApiResponse, Debug)]
pub enum Auth {
    #[oai(status = 200)]
    Ok,
}
