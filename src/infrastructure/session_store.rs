use axum::error_handling::HandleErrorLayer;
use tower::{
    layer,
    ServiceBuilder,
};
use tower_sessions::{
    fred::prelude::*,
    RedisStore,
    SessionManagerLayer,
};

pub type Session = tower_sessions::Session;

pub type LayerParams = (RedisStore, ServiceBuilder<layer::util::Identity>);

pub async fn get_layer_params() -> LayerParams {
    let redis_client = RedisClient::default();
    redis_client.connect();
    redis_client.wait_for_connect().await.unwrap();

    let redis_store = RedisStore::new(redis_client);

    let layer_builder = ServiceBuilder::new();

    (redis_store, layer_builder)
}

type Layer<E, T> = ServiceBuilder<
    layer::util::Stack<
        SessionManagerLayer<RedisStore>,
        layer::util::Stack<HandleErrorLayer<E, T>, layer::util::Identity>,
    >,
>;

pub fn get_layer<E, T>(
    (redis_store, layer_builder): LayerParams,
    handle_error_closure: E,
    manager_closure: impl FnOnce(SessionManagerLayer<RedisStore>) -> SessionManagerLayer<RedisStore>,
) -> Layer<E, T> {
    let session_manager_layer = manager_closure(SessionManagerLayer::new(redis_store));

    layer_builder
        .layer(HandleErrorLayer::new(handle_error_closure))
        .layer(session_manager_layer)
}
