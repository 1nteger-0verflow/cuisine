use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    cuisine::create_app(pool)
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

async fn post_dish(app: &axum::Router, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post("/dishes")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_dishes_returns_array() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/dishes").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(body_json(response).await.is_array());
}

#[tokio::test]
async fn test_create_dish_returns_201() {
    let app = test_app().await;
    let response = post_dish(
        &app,
        json!({ "french": "bouillabaisse", "genre": "stew" }),
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["french"], "bouillabaisse");
    assert_eq!(body["genre"], "stew");
    assert!(body["id"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn test_get_dish_not_found() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/dishes/999").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_get_dish_includes_related_terms() {
    let app = test_app().await;
    let created =
        body_json(post_dish(&app, json!({ "french": "pot-au-feu" })).await)
            .await;
    let id = created["id"].as_i64().unwrap();

    let response =
        app.oneshot(Request::get(format!("/dishes/{id}")).body(Body::empty()).unwrap())
            .await
            .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "pot-au-feu");
    assert!(body["related_terms"].is_array());
}

#[tokio::test]
async fn test_list_dishes_filter_by_genre() {
    let app = test_app().await;
    post_dish(&app, json!({ "french": "crème brûlée", "genre": "dessert" })).await;

    let response = app
        .oneshot(Request::get("/dishes?genre=dessert").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    let arr = body.as_array().unwrap();
    assert!(arr.iter().all(|d| d["genre"] == "dessert"));
}

#[tokio::test]
async fn test_update_dish_partial() {
    let app = test_app().await;
    let created =
        body_json(post_dish(&app, json!({ "french": "crêpe" })).await).await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/dishes/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(json!({ "genre": "pastry" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "crêpe");
    assert_eq!(body["genre"], "pastry");
}

#[tokio::test]
async fn test_delete_dish_returns_204() {
    let app = test_app().await;
    let created =
        body_json(post_dish(&app, json!({ "french": "quiche" })).await).await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/dishes/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}
