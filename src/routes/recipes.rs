use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    db::recipes,
    error::{AppError, AppJson},
    models::recipe::{NewRecipe, Recipe, RecipeDetail, UpdateRecipe},
};

pub async fn list_recipes(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Recipe>>, AppError> {
    let list = recipes::list_recipes(&pool).await?;
    Ok(Json(list))
}

pub async fn get_recipe(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<RecipeDetail>, AppError> {
    let detail = recipes::get_recipe_detail(&pool, id).await?;
    Ok(Json(detail))
}

pub async fn create_recipe(
    State(pool): State<SqlitePool>,
    AppJson(body): AppJson<NewRecipe>,
) -> Result<(StatusCode, Json<Recipe>), AppError> {
    let recipe = recipes::create_recipe(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(recipe)))
}

pub async fn update_recipe(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    AppJson(body): AppJson<UpdateRecipe>,
) -> Result<Json<Recipe>, AppError> {
    let recipe = recipes::update_recipe(&pool, id, body).await?;
    Ok(Json(recipe))
}

pub async fn delete_recipe(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    recipes::delete_recipe(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
