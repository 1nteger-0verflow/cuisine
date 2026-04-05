pub mod db;
pub mod error;
pub mod models;
pub mod routes;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::SqlitePool;

pub fn create_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/terms", get(routes::terms::list_terms).post(routes::terms::create_term))
        .route(
            "/terms/{id}",
            get(routes::terms::get_term)
                .put(routes::terms::update_term)
                .delete(routes::terms::delete_term),
        )
        .route("/recipes", get(routes::recipes::list_recipes).post(routes::recipes::create_recipe))
        .route(
            "/recipes/{id}",
            get(routes::recipes::get_recipe)
                .put(routes::recipes::update_recipe)
                .delete(routes::recipes::delete_recipe),
        )
        .with_state(pool)
}
