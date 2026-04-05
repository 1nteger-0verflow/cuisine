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

async fn post_technique(app: &axum::Router, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post("/techniques")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_techniques_returns_array() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/techniques").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(body_json(response).await.is_array());
}

#[tokio::test]
async fn test_create_technique_returns_201() {
    let app = test_app().await;
    let response = post_technique(
        &app,
        json!({ "french": "monter au beurre", "japanese": "モンテ・オ・ブール", "reading": "モンテ・オ・ブール" }),
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["french"], "monter au beurre");
}

#[tokio::test]
async fn test_get_technique_not_found() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/techniques/999").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_get_technique_includes_related_terms() {
    let app = test_app().await;
    let created = body_json(
        post_technique(&app, json!({ "french": "sauté", "japanese": "ソテー" })).await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(Request::get(format!("/techniques/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "sauté");
    assert!(body["related_terms"].is_array());
}

#[tokio::test]
async fn test_search_techniques() {
    let app = test_app().await;
    post_technique(
        &app,
        json!({ "french": "beurre noisette", "japanese": "ブール・ノワゼット" }),
    )
    .await;
    post_technique(&app, json!({ "french": "beurre blanc", "japanese": "ブール・ブラン" })).await;

    let response = app
        .oneshot(Request::get("/techniques?q=noisette").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let arr = body_json(response).await;
    let arr = arr.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["french"], "beurre noisette");
}

#[tokio::test]
async fn test_delete_technique_returns_204() {
    let app = test_app().await;
    let created =
        body_json(post_technique(&app, json!({ "french": "émulsifier", "japanese": "乳化する" })).await)
            .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/techniques/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}
