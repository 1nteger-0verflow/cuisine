use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::{relations, techniques},
    error::{AppError, AppJson},
    models::relation::{NewRelation, TermCategory},
    models::technique::{NewTechnique, Technique, TechniqueDetail, TechniqueQuery, UpdateTechnique},
};

pub async fn list_techniques(
    State(pool): State<SqlitePool>,
    Query(query): Query<TechniqueQuery>,
) -> Result<Json<Vec<Technique>>, AppError> {
    Ok(Json(techniques::list_techniques(&pool, &query).await?))
}

pub async fn get_technique(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<TechniqueDetail>, AppError> {
    Ok(Json(techniques::get_technique_detail(&pool, id).await?))
}

pub async fn create_technique(
    State(pool): State<SqlitePool>,
    AppJson(body): AppJson<NewTechnique>,
) -> Result<(StatusCode, Json<Technique>), AppError> {
    let technique = techniques::create_technique(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(technique)))
}

pub async fn update_technique(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<UpdateTechnique>,
) -> Result<Json<Technique>, AppError> {
    Ok(Json(techniques::update_technique(&pool, id, body).await?))
}

pub async fn delete_technique(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    techniques::delete_technique(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_relation(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<NewRelation>,
) -> Result<StatusCode, AppError> {
    relations::add_relation(&pool, &TermCategory::Technique, id, body).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_relation(
    State(pool): State<SqlitePool>,
    Path((id, to_cat, to_id)): Path<(i64, TermCategory, i64)>,
) -> Result<StatusCode, AppError> {
    relations::delete_relation(&pool, &TermCategory::Technique, id, &to_cat, to_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
