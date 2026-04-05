use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::{ingredients, relations},
    error::{AppError, AppJson},
    models::ingredient::{Ingredient, IngredientDetail, IngredientQuery, NewIngredient, UpdateIngredient},
    models::relation::{NewRelation, TermCategory},
};

pub async fn list_ingredients(
    State(pool): State<SqlitePool>,
    Query(query): Query<IngredientQuery>,
) -> Result<Json<Vec<Ingredient>>, AppError> {
    Ok(Json(ingredients::list_ingredients(&pool, &query).await?))
}

pub async fn get_ingredient(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<IngredientDetail>, AppError> {
    Ok(Json(ingredients::get_ingredient_detail(&pool, id).await?))
}

pub async fn create_ingredient(
    State(pool): State<SqlitePool>,
    AppJson(body): AppJson<NewIngredient>,
) -> Result<(StatusCode, Json<Ingredient>), AppError> {
    let ingredient = ingredients::create_ingredient(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(ingredient)))
}

pub async fn update_ingredient(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<UpdateIngredient>,
) -> Result<Json<Ingredient>, AppError> {
    Ok(Json(ingredients::update_ingredient(&pool, id, body).await?))
}

pub async fn delete_ingredient(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    ingredients::delete_ingredient(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_relation(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<NewRelation>,
) -> Result<StatusCode, AppError> {
    relations::add_relation(&pool, &TermCategory::Ingredient, id, body).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_relation(
    State(pool): State<SqlitePool>,
    Path((id, to_cat, to_id)): Path<(i64, TermCategory, i64)>,
) -> Result<StatusCode, AppError> {
    relations::delete_relation(&pool, &TermCategory::Ingredient, id, &to_cat, to_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
