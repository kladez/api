#![allow(dead_code, unused)]
#![feature(never_type)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

use std::time::Duration;

use application::{
    dtos,
    get_api_service,
};
use poem::{
    listener::TcpListener,
    middleware::Cors,
    session::{
        CookieConfig,
        RedisStorage,
        ServerSession,
    },
    web::cookie::CookieKey,
    EndpointExt,
    IntoEndpoint,
    Route,
    Server,
};
use poem_openapi::payload::Json;

pub mod application;
mod config;
mod domain;
pub mod infrastructure;

pub use config::Config;
use infrastructure::Infrastructure;

pub async fn get_app(config: &Config) -> impl IntoEndpoint {
    let infrastructure = Infrastructure::new(config).await;

    let server_session = ServerSession::new(
        CookieConfig::private(CookieKey::generate())
            .name("session")
            .domain("localhost")
            .path("/")
            .same_site(poem::web::cookie::SameSite::None)
            .secure(true)
            .http_only(true)
            .max_age(Duration::from_secs(30 * 24 * 60 * 60)),
        RedisStorage::new(infrastructure::new_redis_storage(config).await),
    );

    let api_service = get_api_service(config);

    let swagger = api_service.swagger_ui();

    Route::new()
        .at("/summary", application::handlers::summary::handler)
        .at("/metrics", infrastructure::prometheus_metrics)
        .nest("/", api_service)
        .nest("/swagger", swagger)
        .catch_error(|_: poem::error::NotFoundError| async {
            dtos::Error::NotFound(Json("endpoint not found".to_string().into()))
        })
        .data(infrastructure)
        .with(Cors::new().allow_credentials(true))
        .with(server_session)
        .with(infrastructure::PrometheusMetrics)
}

pub async fn run() -> Result<!, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = Config::new();

    let app = get_app(&config).await;

    Server::new(TcpListener::bind(&config.host))
        .run(app)
        .await?;

    Err("server exited unexpectedly".into())
}
