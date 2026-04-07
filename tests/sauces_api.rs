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

async fn post_sauce(app: &axum::Router, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post("/sauces")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_sauces_returns_array() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/sauces").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(body_json(response).await.is_array());
}

#[tokio::test]
async fn test_create_sauce_returns_201() {
    let app = test_app().await;
    let response = post_sauce(
        &app,
        json!({ "french": "béchamel", "genre": "mere" }),
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["french"], "béchamel");
    assert_eq!(body["genre"], "mere");
    assert!(body["id"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn test_get_sauce_not_found() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/sauces/999").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_get_sauce_includes_related_terms() {
    let app = test_app().await;
    let created =
        body_json(post_sauce(&app, json!({ "french": "velouté" })).await).await;
    let id = created["id"].as_i64().unwrap();

    let response =
        app.oneshot(Request::get(format!("/sauces/{id}")).body(Body::empty()).unwrap())
            .await
            .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "velouté");
    assert!(body["related_terms"].is_array());
}

#[tokio::test]
async fn test_list_sauces_filter_by_genre() {
    let app = test_app().await;
    post_sauce(&app, json!({ "french": "hollandaise", "genre": "emulsionnee" })).await;

    let response = app
        .oneshot(Request::get("/sauces?genre=emulsionnee").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    let arr = body.as_array().unwrap();
    assert!(arr.iter().all(|s| s["genre"] == "emulsionnee"));
}

#[tokio::test]
async fn test_update_sauce_partial() {
    let app = test_app().await;
    let created =
        body_json(post_sauce(&app, json!({ "french": "espagnole" })).await).await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/sauces/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(json!({ "genre": "mere" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "espagnole");
    assert_eq!(body["genre"], "mere");
}

#[tokio::test]
async fn test_delete_sauce_returns_204() {
    let app = test_app().await;
    let created =
        body_json(post_sauce(&app, json!({ "french": "mayonnaise" })).await).await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/sauces/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}
