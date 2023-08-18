#![allow(dead_code, unused)]

use poem::{
    http::{
        header,
        StatusCode,
    },
    test::TestClient,
};

#[tokio::test]
async fn test() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = kladez_api::Config::new();
    let app = kladez_api::get_app(&config).await;
    let cli = TestClient::new(app);

    // GET /users
    let resp = cli.get("/users").send().await;

    resp.assert_status(StatusCode::OK);
    resp.assert_content_type("application/json; charset=utf-8");
    resp.assert_text("[]").await;

    // POST /users
    let resp = cli
        .post("/users")
        .body_json(&serde_json::json!({
            "name": "admin",
            "email": "admin@example.com",
            "password": "password"
        }))
        .send()
        .await;

    resp.assert_status(StatusCode::CREATED);
    resp.assert_text("").await;

    // POST /users
    let resp = cli
        .post("/users")
        .body_json(&serde_json::json!({
            "name": "admin",
            "email": "admin_@example.com",
            "password": "password"
        }))
        .send()
        .await;

    resp.assert_status(StatusCode::BAD_REQUEST);
    resp.assert_json(serde_json::json!({
        "message": "name is already registered",
    }))
    .await;

    // POST /users
    let resp = cli
        .post("/users")
        .body_json(&serde_json::json!({
            "name": "admin_",
            "email": "admin@example.com",
            "password": "password"
        }))
        .send()
        .await;

    resp.assert_status(StatusCode::BAD_REQUEST);
    resp.assert_json(serde_json::json!({
        "message": "email is already registered",
    }))
    .await;

    // GET /users
    let resp = cli.get("/users").send().await;

    resp.assert_status(StatusCode::OK);
    resp.assert_json(serde_json::json!([
        {
            "name": "admin",
        },
    ]))
    .await;

    // GET /auth
    let resp = cli
        .post("/auth")
        .body_json(&serde_json::json!({
            "name": "admin",
            "password": "password"
        }))
        .send()
        .await;

    let cookie = resp.0.header(header::SET_COOKIE).unwrap().to_owned();

    resp.assert_status(StatusCode::OK);
    resp.assert_header_exist(header::SET_COOKIE);
    resp.assert_text("").await;

    // POST /users/api-keys
    let resp = cli
        .post("/users/api-keys")
        .header(header::COOKIE, cookie)
        .body_json(&serde_json::json!({
            "name": "api key 1",
            "valid_until": "2023-11-11T10:10:10",
        }))
        .send()
        .await;

    resp.assert_status(StatusCode::CREATED);
    let data = resp.json().await;
    let data = data.value().object();
    let api_key = data.get("api_key").string();
    assert_eq!(data.get("api_key").string().len(), 128);
}
