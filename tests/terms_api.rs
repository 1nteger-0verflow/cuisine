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

async fn post_term(app: &axum::Router, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post("/terms")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_terms_empty() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/terms").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_create_term_returns_201() {
    let app = test_app().await;
    let response = post_term(
        &app,
        json!({ "french": "beurre", "japanese": "バター", "category": "ingredient" }),
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["french"], "beurre");
    assert_eq!(body["japanese"], "バター");
    assert!(body["id"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn test_create_term_invalid_category_returns_400() {
    let app = test_app().await;
    let response = post_term(
        &app,
        json!({ "french": "beurre", "japanese": "バター", "category": "invalid" }),
    )
    .await;
    assert_eq!(response.status(), 400);
    let body = body_json(response).await;
    assert!(body["error"].as_str().is_some());
}

#[tokio::test]
async fn test_get_term_not_found() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/terms/999").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
    let body = body_json(response).await;
    assert_eq!(body["error"], "not found");
}

#[tokio::test]
async fn test_list_terms_filter_by_category() {
    let app = test_app().await;
    post_term(&app, json!({ "french": "beurre", "japanese": "バター", "category": "ingredient" }))
        .await;
    post_term(
        &app,
        json!({ "french": "bouillabaisse", "japanese": "ブイヤベース", "category": "dish" }),
    )
    .await;

    let response = app
        .oneshot(Request::get("/terms?category=ingredient").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    let arr = body.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["french"], "beurre");
}

#[tokio::test]
async fn test_list_terms_search() {
    let app = test_app().await;
    post_term(
        &app,
        json!({ "french": "beurre noisette", "japanese": "ブール・ノワゼット", "category": "technique" }),
    )
    .await;
    post_term(
        &app,
        json!({ "french": "beurre blanc", "japanese": "ブール・ブラン", "category": "technique" }),
    )
    .await;

    let response = app
        .oneshot(Request::get("/terms?q=noisette").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    let arr = body.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["french"], "beurre noisette");
}

#[tokio::test]
async fn test_update_term_partial() {
    let app = test_app().await;
    let created = body_json(
        post_term(&app, json!({ "french": "beurre", "japanese": "バター", "category": "ingredient" }))
            .await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/terms/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(json!({ "japanese": "澄ましバター" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "beurre");
    assert_eq!(body["japanese"], "澄ましバター");
}

#[tokio::test]
async fn test_delete_term_returns_204() {
    let app = test_app().await;
    let created = body_json(
        post_term(&app, json!({ "french": "beurre", "japanese": "バター", "category": "ingredient" }))
            .await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/terms/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}

#[tokio::test]
async fn test_crud_round_trip() {
    let app = test_app().await;

    // Create
    let created = body_json(
        post_term(&app, json!({ "french": "beurre", "japanese": "バター", "category": "ingredient" }))
            .await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    // Read
    let fetched = body_json(
        app.clone()
            .oneshot(Request::get(format!("/terms/{id}")).body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(fetched["french"], "beurre");

    // Update
    let updated = body_json(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/terms/{id}"))
                    .header("content-type", "application/json")
                    .body(Body::from(json!({ "notes": "fond de sauce" }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(updated["notes"], "fond de sauce");
    assert_eq!(updated["french"], "beurre");

    // Delete
    let del = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/terms/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del.status(), 204);

    // Verify 404
    let not_found = app
        .oneshot(Request::get(format!("/terms/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(not_found.status(), 404);
}
