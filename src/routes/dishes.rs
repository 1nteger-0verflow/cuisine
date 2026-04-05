use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::{dishes, relations},
    error::{AppError, AppJson},
    models::dish::{Dish, DishDetail, DishQuery, NewDish, UpdateDish},
    models::relation::{NewRelation, TermCategory},
};

pub async fn list_dishes(
    State(pool): State<SqlitePool>,
    Query(query): Query<DishQuery>,
) -> Result<Json<Vec<Dish>>, AppError> {
    Ok(Json(dishes::list_dishes(&pool, &query).await?))
}

pub async fn get_dish(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<DishDetail>, AppError> {
    Ok(Json(dishes::get_dish_detail(&pool, id).await?))
}

pub async fn create_dish(
    State(pool): State<SqlitePool>,
    AppJson(body): AppJson<NewDish>,
) -> Result<(StatusCode, Json<Dish>), AppError> {
    let dish = dishes::create_dish(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(dish)))
}

pub async fn update_dish(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<UpdateDish>,
) -> Result<Json<Dish>, AppError> {
    Ok(Json(dishes::update_dish(&pool, id, body).await?))
}

pub async fn delete_dish(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    dishes::delete_dish(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_relation(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<NewRelation>,
) -> Result<StatusCode, AppError> {
    relations::add_relation(&pool, &TermCategory::Dish, id, body).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_relation(
    State(pool): State<SqlitePool>,
    Path((id, to_cat, to_id)): Path<(i64, TermCategory, i64)>,
) -> Result<StatusCode, AppError> {
    relations::delete_relation(&pool, &TermCategory::Dish, id, &to_cat, to_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
