pub mod user;

pub fn get_router() -> axum::Router<crate::infrastructure::database::Pool> {
    axum::Router::new()
        .route("/health-check", axum::routing::head(|| async {}))
        .nest("/", user::get_router())
}
