use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::terms,
    error::AppError,
    models::term::{NewTerm, Term, TermQuery, UpdateTerm},
};

pub async fn list_terms(
    State(pool): State<SqlitePool>,
    Query(query): Query<TermQuery>,
) -> Result<Json<Vec<Term>>, AppError> {
    let terms = terms::list_terms(&pool, &query).await?;
    Ok(Json(terms))
}

pub async fn get_term(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Term>, AppError> {
    let term = terms::get_term(&pool, id).await?;
    Ok(Json(term))
}

pub async fn create_term(
    State(pool): State<SqlitePool>,
    Json(body): Json<NewTerm>,
) -> Result<(StatusCode, Json<Term>), AppError> {
    let term = terms::create_term(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(term)))
}

pub async fn update_term(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateTerm>,
) -> Result<Json<Term>, AppError> {
    let term = terms::update_term(&pool, id, body).await?;
    Ok(Json(term))
}

pub async fn delete_term(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    terms::delete_term(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
