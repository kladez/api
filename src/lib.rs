#![allow(dead_code, unused)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

use std::time::Duration;

use application::{
    dtos,
    get_api_service,
};
use poem::{
    listener::TcpListener,
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
mod domain;
pub mod infrastructure;

use infrastructure::Infrastructure;

#[derive(Debug)]
pub struct Config {
    host: String,
    db_url: String,
    db_max_connections: u32,
    kafka_brokers: String,
    redis_url: String,
}

impl Config {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            host: std::env::var("HOST").expect("HOST must be set"),
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            db_max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .expect("DATABASE_MAX_CONNECTIONS must be set")
                .parse::<u32>()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            kafka_brokers: std::env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS must be set"),
            redis_url: std::env::var("REDIS_URL").expect("REDIS_URL must be set"),
        }
    }
}

pub async fn get_app(config: &Config) -> impl IntoEndpoint {
    let infrastructure = Infrastructure::new(config).await;

    let server_session = ServerSession::new(
        CookieConfig::private(CookieKey::generate())
            .name("session")
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(Duration::from_secs(30 * 24 * 60 * 60)),
        RedisStorage::new(infrastructure::new_redis_storage(config).await),
    );

    let api_service = get_api_service(config);

    let swagger = api_service.swagger_ui();

    let app = Route::new()
        .at("/summary", application::handlers::summary::summary)
        .at("/metrics", infrastructure::prometheus_metrics)
        .nest("/", api_service)
        .nest("/swagger", swagger)
        .catch_error(|_: poem::error::NotFoundError| async {
            dtos::Error::NotFound(Json("endpoint not found".to_string().into()))
        })
        .data(infrastructure)
        .with(server_session)
        .with(infrastructure::PrometheusMetrics);

    app
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = Config::new();

    let app = get_app(&config).await;

    Server::new(TcpListener::bind(&config.host))
        .run(app)
        .await?;

    Ok(())
}
