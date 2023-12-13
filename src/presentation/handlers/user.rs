use axum::{
    debug_handler,
    extract::{
        Path,
        Query,
        State,
    },
    http::StatusCode,
    routing,
    Json,
    Router,
};

use crate::{
    application::Session,
    domain::services,
    infrastructure,
    presentation::dtos,
};

#[debug_handler]
pub async fn create(
    State(database_pool): State<infrastructure::database::Pool>,
    Json(user): Json<dtos::user::CreateRequest>,
) -> Result<(), (StatusCode, String)> {
    let service = services::User::new(database_pool);

    service
        .create(&user)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))
}

#[debug_handler]
async fn get_all(
    State(database_pool): State<infrastructure::database::Pool>,
    session: Session,
    Query(pagination): Query<dtos::pagination::Pagination<dtos::user::OrderBy>>,
) -> Result<Json<Vec<dtos::user::GetResponse>>, (StatusCode, String)> {
    if let Ok(Some::<i32>(id)) = session.get("id") {
        tracing::info!("User ID: {}", id);
        session
            .insert("id", id + 1)
            .expect("Failed to insert user ID into session");
    } else {
        tracing::info!("No user ID");
        session.insert("id", 0).expect("Failed to insert user ID into session");
    }

    let service = services::User::new(database_pool);

    service
        .get_all(&pagination)
        .await
        .map(Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))
}

#[debug_handler]
async fn get(
    State(database_pool): State<infrastructure::database::Pool>,
    Path(id): Path<i32>,
) -> Result<Json<dtos::user::GetResponse>, (StatusCode, String)> {
    let service = services::User::new(database_pool);

    service
        .get(&id)
        .await
        .map(Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))
}

#[debug_handler]
async fn update(
    State(database_pool): State<infrastructure::database::Pool>,
    Path(id): Path<i32>,
    Json(user): Json<dtos::user::UpdateRequest>,
) -> Result<(), (StatusCode, String)> {
    let service = services::User::new(database_pool);

    service
        .update(&id, &user)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))
}

#[debug_handler]
async fn delete(
    State(database_pool): State<infrastructure::database::Pool>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    let service = services::User::new(database_pool);

    service
        .delete(&id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))
}

pub fn get_router() -> axum::Router<infrastructure::database::Pool> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/", routing::post(create).get(get_all))
            .route("/:id", routing::get(get).patch(update).delete(delete)),
    )
}
