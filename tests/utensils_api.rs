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

async fn post_utensil(app: &axum::Router, body: Value) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::post("/utensils")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_list_utensils_returns_array() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/utensils").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(body_json(response).await.is_array());
}

#[tokio::test]
async fn test_create_utensil_returns_201() {
    let app = test_app().await;
    let response = post_utensil(
        &app,
        json!({ "french": "chinois", "japanese": "シノワ", "reading": "シノワ", "notes": "円錐形の裏ごし器" }),
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = body_json(response).await;
    assert_eq!(body["french"], "chinois");
    assert_eq!(body["reading"], "シノワ");
}

#[tokio::test]
async fn test_get_utensil_not_found() {
    let app = test_app().await;
    let response =
        app.oneshot(Request::get("/utensils/999").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_relation_cross_category() {
    let app = test_app().await;

    // Create utensil and technique
    let utensil = body_json(
        post_utensil(&app, json!({ "french": "tamis", "japanese": "タミ" })).await,
    )
    .await;
    let u_id = utensil["id"].as_i64().unwrap();

    let technique = body_json(
        app.clone()
            .oneshot(
                Request::post("/techniques")
                    .header("content-type", "application/json")
                    .body(
                        Body::from(
                            json!({ "french": "tamiser", "japanese": "裏ごしする" }).to_string(),
                        ),
                    )
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;
    let t_id = technique["id"].as_i64().unwrap();

    // Add cross-category relation
    let rel_resp = app
        .clone()
        .oneshot(
            Request::post(format!("/utensils/{u_id}/relations"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "to_category": "technique", "to_id": t_id }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(rel_resp.status(), 201);

    // Verify relation from utensil side
    let detail = body_json(
        app.clone()
            .oneshot(Request::get(format!("/utensils/{u_id}")).body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    let related = detail["related_terms"].as_array().unwrap();
    assert_eq!(related.len(), 1);
    assert_eq!(related[0]["french"], "tamiser");
    assert_eq!(related[0]["category"], "technique");

    // Verify bidirectional: from technique side
    let t_detail = body_json(
        app.clone()
            .oneshot(Request::get(format!("/techniques/{t_id}")).body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    let t_related = t_detail["related_terms"].as_array().unwrap();
    assert_eq!(t_related.len(), 1);
    assert_eq!(t_related[0]["french"], "tamis");
    assert_eq!(t_related[0]["category"], "utensil");

    // Delete relation
    let del = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/utensils/{u_id}/relations/technique/{t_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del.status(), 204);
}

#[tokio::test]
async fn test_delete_utensil_returns_204() {
    let app = test_app().await;
    let created =
        body_json(post_utensil(&app, json!({ "french": "louche", "japanese": "レードル" })).await)
            .await;
    let id = created["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/utensils/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}
