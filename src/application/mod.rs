use poem_openapi::OpenApiService;

use crate::Config;

pub mod dtos;
pub mod handlers;

#[derive(Debug, poem_openapi::Tags)]
pub enum Tags {
    ApiKey,
    Auth,
    User,
}

macro_rules! get_api_service {
    ($($path:path),* $(,)?) => {
        pub fn get_api_service(config: &Config) -> OpenApiService<($($path,)*), ()> {
            let handlers = ($($path,)*);
            let version = env!("CARGO_PKG_VERSION");
            let license = env!("CARGO_PKG_LICENSE");
            let scheme = if cfg!(debug_assertions) { "http" } else { "https" };
            OpenApiService::new(handlers, "Kladez API", version)
                .license(license)
                .server(format!("{scheme}://{}", &config.host))
        }
    };
}

get_api_service!(
    handlers::user::Api,
    handlers::user::api_key::Api,
    handlers::auth::Api,
);
