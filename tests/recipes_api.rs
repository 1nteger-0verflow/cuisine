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

async fn post_json(app: &axum::Router, uri: &str, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_recipes_empty() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/recipes").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_create_recipe_returns_201() {
    let app = test_app().await;
    let response =
        post_json(&app, "/recipes", json!({ "name_french": "bouillabaisse", "difficulty": "hard" }))
            .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["name_french"], "bouillabaisse");
    assert_eq!(body["difficulty"], "hard");
    assert!(body["id"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn test_get_recipe_not_found() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/recipes/999").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
    let body = body_json(response).await;
    assert_eq!(body["error"], "not found");
}

#[tokio::test]
async fn test_get_recipe_detail_shape() {
    let app = test_app().await;
    let created =
        body_json(post_json(&app, "/recipes", json!({ "name_french": "bouillabaisse" })).await)
            .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(Request::get(format!("/recipes/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["name_french"], "bouillabaisse");
    assert_eq!(body["ingredients"], json!([]));
    assert_eq!(body["steps"], json!([]));
}

#[tokio::test]
async fn test_delete_recipe_returns_204() {
    let app = test_app().await;
    let created =
        body_json(post_json(&app, "/recipes", json!({ "name_french": "bouillabaisse" })).await)
            .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/recipes/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}

#[tokio::test]
async fn test_recipe_crud_round_trip() {
    let app = test_app().await;

    // Create
    let created = body_json(
        post_json(
            &app,
            "/recipes",
            json!({ "name_french": "bouillabaisse", "difficulty": "hard" }),
        )
        .await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    // Update
    let updated = body_json(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/recipes/{id}"))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({ "description_japanese": "マルセイユの魚介スープ" }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(updated["name_french"], "bouillabaisse");
    assert_eq!(updated["description_japanese"], "マルセイユの魚介スープ");
    assert_eq!(updated["difficulty"], "hard");

    // Delete
    let del = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/recipes/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del.status(), 204);

    // Verify 404
    let not_found = app
        .oneshot(Request::get(format!("/recipes/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(not_found.status(), 404);
}
