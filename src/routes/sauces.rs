use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::{relations, sauces},
    error::{AppError, AppJson},
    models::sauce::{NewSauce, Sauce, SauceDetail, SauceQuery, UpdateSauce},
    models::relation::{NewRelation, TermCategory},
};

pub async fn list_sauces(
    State(pool): State<SqlitePool>,
    Query(query): Query<SauceQuery>,
) -> Result<Json<Vec<Sauce>>, AppError> {
    Ok(Json(sauces::list_sauces(&pool, &query).await?))
}

pub async fn get_sauce(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<SauceDetail>, AppError> {
    Ok(Json(sauces::get_sauce_detail(&pool, id).await?))
}

pub async fn create_sauce(
    State(pool): State<SqlitePool>,
    AppJson(body): AppJson<NewSauce>,
) -> Result<(StatusCode, Json<Sauce>), AppError> {
    let sauce = sauces::create_sauce(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(sauce)))
}

pub async fn update_sauce(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<UpdateSauce>,
) -> Result<Json<Sauce>, AppError> {
    Ok(Json(sauces::update_sauce(&pool, id, body).await?))
}

pub async fn delete_sauce(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    sauces::delete_sauce(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_relation(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<NewRelation>,
) -> Result<StatusCode, AppError> {
    relations::add_relation(&pool, &TermCategory::Sauce, id, body).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_relation(
    State(pool): State<SqlitePool>,
    Path((id, to_cat, to_id)): Path<(i64, TermCategory, i64)>,
) -> Result<StatusCode, AppError> {
    relations::delete_relation(&pool, &TermCategory::Sauce, id, &to_cat, to_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
