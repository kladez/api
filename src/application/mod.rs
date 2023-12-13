use axum::{
    http::StatusCode,
    BoxError,
};
use tokio::net::TcpListener;
use tower_sessions::Expiry;

use crate::{
    infrastructure,
    presentation::handlers,
};

pub mod config;

pub type Session = infrastructure::session_store::Session;

pub async fn run(
    config: &config::Config,
    tracing_layer: infrastructure::tracing::Layer,
    session_store_layer_params: infrastructure::session_store::LayerParams,
    database_pool: infrastructure::database::Pool,
) {
    let session_store_layer = infrastructure::session_store::get_layer(
        session_store_layer_params,
        |error: BoxError| async move {
            tracing::error!(error = %error, "Error handling request");
            // (
            //     StatusCode::BAD_REQUEST,
            //     axum::Json(
            //         &serde_json::json!({
            //             "error": error.to_string(),
            //         }),
            //     ),
            // )
            StatusCode::BAD_REQUEST
        },
        |session_manager_layer| {
            session_manager_layer
                .with_name("session")
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(time::Duration::seconds(60)))
        },
    );

    let app = axum::Router::new()
        .nest("/", handlers::get_router())
        .layer(tracing_layer)
        .layer(session_store_layer)
        .with_state(database_pool);

    tracing::info!(address = %config.bind_address, "Starting server");

    let listener = TcpListener::bind(&config.bind_address).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
