use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::{relations, utensils},
    error::{AppError, AppJson},
    models::relation::{NewRelation, TermCategory},
    models::utensil::{NewUtensil, UpdateUtensil, Utensil, UtensilDetail, UtensilQuery},
};

pub async fn list_utensils(
    State(pool): State<SqlitePool>,
    Query(query): Query<UtensilQuery>,
) -> Result<Json<Vec<Utensil>>, AppError> {
    Ok(Json(utensils::list_utensils(&pool, &query).await?))
}

pub async fn get_utensil(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<UtensilDetail>, AppError> {
    Ok(Json(utensils::get_utensil_detail(&pool, id).await?))
}

pub async fn create_utensil(
    State(pool): State<SqlitePool>,
    AppJson(body): AppJson<NewUtensil>,
) -> Result<(StatusCode, Json<Utensil>), AppError> {
    let utensil = utensils::create_utensil(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(utensil)))
}

pub async fn update_utensil(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<UpdateUtensil>,
) -> Result<Json<Utensil>, AppError> {
    Ok(Json(utensils::update_utensil(&pool, id, body).await?))
}

pub async fn delete_utensil(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    utensils::delete_utensil(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_relation(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<NewRelation>,
) -> Result<StatusCode, AppError> {
    relations::add_relation(&pool, &TermCategory::Utensil, id, body).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_relation(
    State(pool): State<SqlitePool>,
    Path((id, to_cat, to_id)): Path<(i64, TermCategory, i64)>,
) -> Result<StatusCode, AppError> {
    relations::delete_relation(&pool, &TermCategory::Utensil, id, &to_cat, to_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
