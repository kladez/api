mod application;
mod domain;
mod helpers;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    infrastructure::tracing::init();

    let config = application
        ::config
        ::Config::new();

    let tracing_layer = infrastructure::tracing::get_layer();

    let session_store_layer_params = infrastructure::session_store::get_layer_params().await;

    let databasePool = infrastructure::database::get_pool(&config).await;

    application::run(&config, tracing_layer, session_store_layer_params, databasePool).await;
}
