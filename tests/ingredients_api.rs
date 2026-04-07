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

async fn post_ingredient(app: &axum::Router, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_ingredients_returns_array() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/ingredients").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(body_json(response).await.is_array());
}

#[tokio::test]
async fn test_create_ingredient_returns_201() {
    let app = test_app().await;
    let response = post_ingredient(
        &app,
        json!({ "french": "safran", "genre": "spice", "reading": "サフラン" }),
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["french"], "safran");
    assert_eq!(body["genre"], "spice");
    assert_eq!(body["reading"], "サフラン");
}

#[tokio::test]
async fn test_get_ingredient_not_found() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/ingredients/999").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_get_ingredient_includes_related_terms() {
    let app = test_app().await;
    let created = body_json(
        post_ingredient(&app, json!({ "french": "beurre" })).await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(Request::get(format!("/ingredients/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_json(response).await;
    assert_eq!(body["french"], "beurre");
    assert!(body["related_terms"].is_array());
}

#[tokio::test]
async fn test_list_ingredients_filter_by_genre() {
    let app = test_app().await;
    post_ingredient(&app, json!({ "french": "thym", "genre": "herb" })).await;

    let response = app
        .oneshot(Request::get("/ingredients?genre=herb").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let arr = body_json(response).await;
    let arr = arr.as_array().unwrap();
    assert!(arr.iter().all(|i| i["genre"] == "herb"));
}

#[tokio::test]
async fn test_crud_round_trip() {
    let app = test_app().await;

    let created = body_json(
        post_ingredient(
            &app,
            json!({ "french": "safran", "genre": "spice" }),
        )
        .await,
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    // Read
    let fetched = body_json(
        app.clone()
            .oneshot(Request::get(format!("/ingredients/{id}")).body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(fetched["french"], "safran");

    // Update
    let updated = body_json(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/ingredients/{id}"))
                    .header("content-type", "application/json")
                    .body(Body::from(json!({ "notes": "高級スパイス" }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(updated["notes"], "高級スパイス");
    assert_eq!(updated["french"], "safran");

    // Delete
    let del = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/ingredients/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del.status(), 204);

    let not_found = app
        .oneshot(Request::get(format!("/ingredients/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(not_found.status(), 404);
}
