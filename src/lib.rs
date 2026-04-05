pub mod db;
pub mod error;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};
use sqlx::SqlitePool;

pub fn create_app(pool: SqlitePool) -> Router {
    Router::new()
        // dishes
        .route("/dishes", get(routes::dishes::list_dishes).post(routes::dishes::create_dish))
        .route(
            "/dishes/{id}",
            get(routes::dishes::get_dish)
                .put(routes::dishes::update_dish)
                .delete(routes::dishes::delete_dish),
        )
        .route("/dishes/{id}/relations", axum::routing::post(routes::dishes::add_relation))
        .route(
            "/dishes/{id}/relations/{to_cat}/{to_id}",
            axum::routing::delete(routes::dishes::delete_relation),
        )
        // ingredients
        .route(
            "/ingredients",
            get(routes::ingredients::list_ingredients).post(routes::ingredients::create_ingredient),
        )
        .route(
            "/ingredients/{id}",
            get(routes::ingredients::get_ingredient)
                .put(routes::ingredients::update_ingredient)
                .delete(routes::ingredients::delete_ingredient),
        )
        .route(
            "/ingredients/{id}/relations",
            axum::routing::post(routes::ingredients::add_relation),
        )
        .route(
            "/ingredients/{id}/relations/{to_cat}/{to_id}",
            axum::routing::delete(routes::ingredients::delete_relation),
        )
        // utensils
        .route(
            "/utensils",
            get(routes::utensils::list_utensils).post(routes::utensils::create_utensil),
        )
        .route(
            "/utensils/{id}",
            get(routes::utensils::get_utensil)
                .put(routes::utensils::update_utensil)
                .delete(routes::utensils::delete_utensil),
        )
        .route(
            "/utensils/{id}/relations",
            axum::routing::post(routes::utensils::add_relation),
        )
        .route(
            "/utensils/{id}/relations/{to_cat}/{to_id}",
            axum::routing::delete(routes::utensils::delete_relation),
        )
        // techniques
        .route(
            "/techniques",
            get(routes::techniques::list_techniques).post(routes::techniques::create_technique),
        )
        .route(
            "/techniques/{id}",
            get(routes::techniques::get_technique)
                .put(routes::techniques::update_technique)
                .delete(routes::techniques::delete_technique),
        )
        .route(
            "/techniques/{id}/relations",
            axum::routing::post(routes::techniques::add_relation),
        )
        .route(
            "/techniques/{id}/relations/{to_cat}/{to_id}",
            axum::routing::delete(routes::techniques::delete_relation),
        )
        // recipes
        .route(
            "/recipes",
            get(routes::recipes::list_recipes).post(routes::recipes::create_recipe),
        )
        .route(
            "/recipes/{id}",
            get(routes::recipes::get_recipe)
                .put(routes::recipes::update_recipe)
                .delete(routes::recipes::delete_recipe),
        )
        .with_state(pool)
}
