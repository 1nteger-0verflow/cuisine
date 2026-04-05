use sqlx::SqlitePool;

use crate::{
    db::relations,
    error::AppError,
    models::ingredient::{Ingredient, IngredientDetail, IngredientQuery, NewIngredient, UpdateIngredient},
    models::relation::TermCategory,
};

pub async fn list_ingredients(
    pool: &SqlitePool,
    query: &IngredientQuery,
) -> Result<Vec<Ingredient>, AppError> {
    let ingredients = match (&query.genre, &query.q) {
        (Some(g), Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Ingredient,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM ingredients
                   WHERE genre = ? AND (french LIKE ? OR japanese LIKE ?)
                   ORDER BY french"#,
                g, pattern, pattern
            )
            .fetch_all(pool)
            .await?
        }
        (Some(g), None) => {
            sqlx::query_as!(
                Ingredient,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM ingredients WHERE genre = ? ORDER BY french"#,
                g
            )
            .fetch_all(pool)
            .await?
        }
        (None, Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Ingredient,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM ingredients
                   WHERE french LIKE ? OR japanese LIKE ?
                   ORDER BY french"#,
                pattern, pattern
            )
            .fetch_all(pool)
            .await?
        }
        (None, None) => {
            sqlx::query_as!(
                Ingredient,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM ingredients ORDER BY french"#
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(ingredients)
}

pub async fn get_ingredient(pool: &SqlitePool, id: i64) -> Result<Ingredient, AppError> {
    let ingredient: Option<Ingredient> = sqlx::query_as!(
        Ingredient,
        r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
           FROM ingredients WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    ingredient.ok_or(AppError::NotFound)
}

pub async fn get_ingredient_detail(
    pool: &SqlitePool,
    id: i64,
) -> Result<IngredientDetail, AppError> {
    let ingredient = get_ingredient(pool, id).await?;
    let related_terms =
        relations::get_related_terms(pool, &TermCategory::Ingredient, id).await?;
    Ok(IngredientDetail { ingredient, related_terms })
}

pub async fn create_ingredient(
    pool: &SqlitePool,
    new: NewIngredient,
) -> Result<Ingredient, AppError> {
    let ingredient = sqlx::query_as!(
        Ingredient,
        r#"INSERT INTO ingredients (french, japanese, reading, genre, notes)
           VALUES (?, ?, ?, ?, ?)
           RETURNING id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at"#,
        new.french, new.japanese, new.reading, new.genre, new.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(ingredient)
}

pub async fn update_ingredient(
    pool: &SqlitePool,
    id: i64,
    update: UpdateIngredient,
) -> Result<Ingredient, AppError> {
    let existing = get_ingredient(pool, id).await?;
    let french   = update.french.unwrap_or(existing.french);
    let japanese = update.japanese.unwrap_or(existing.japanese);
    let reading  = update.reading.or(existing.reading);
    let genre    = update.genre.or(existing.genre);
    let notes    = update.notes.or(existing.notes);

    let ingredient = sqlx::query_as!(
        Ingredient,
        r#"UPDATE ingredients SET french=?, japanese=?, reading=?, genre=?, notes=?
           WHERE id=?
           RETURNING id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at"#,
        french, japanese, reading, genre, notes, id
    )
    .fetch_one(pool)
    .await?;
    Ok(ingredient)
}

pub async fn delete_ingredient(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM ingredients WHERE id = ?", id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
