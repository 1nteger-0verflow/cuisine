use sqlx::SqlitePool;

use crate::{
    db::relations,
    error::AppError,
    models::dish::{Dish, DishDetail, DishQuery, NewDish, UpdateDish},
    models::relation::TermCategory,
};

pub async fn list_dishes(pool: &SqlitePool, query: &DishQuery) -> Result<Vec<Dish>, AppError> {
    let dishes = match (&query.genre, &query.q) {
        (Some(g), Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Dish,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM dishes
                   WHERE genre = ? AND (french LIKE ? OR japanese LIKE ?)
                   ORDER BY french"#,
                g, pattern, pattern
            )
            .fetch_all(pool)
            .await?
        }
        (Some(g), None) => {
            sqlx::query_as!(
                Dish,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM dishes WHERE genre = ? ORDER BY french"#,
                g
            )
            .fetch_all(pool)
            .await?
        }
        (None, Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Dish,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM dishes
                   WHERE french LIKE ? OR japanese LIKE ?
                   ORDER BY french"#,
                pattern, pattern
            )
            .fetch_all(pool)
            .await?
        }
        (None, None) => {
            sqlx::query_as!(
                Dish,
                r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
                   FROM dishes ORDER BY french"#
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(dishes)
}

pub async fn get_dish(pool: &SqlitePool, id: i64) -> Result<Dish, AppError> {
    let dish: Option<Dish> = sqlx::query_as!(
        Dish,
        r#"SELECT id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at
           FROM dishes WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    dish.ok_or(AppError::NotFound)
}

pub async fn get_dish_detail(pool: &SqlitePool, id: i64) -> Result<DishDetail, AppError> {
    let dish = get_dish(pool, id).await?;
    let related_terms = relations::get_related_terms(pool, &TermCategory::Dish, id).await?;
    Ok(DishDetail { dish, related_terms })
}

pub async fn create_dish(pool: &SqlitePool, new: NewDish) -> Result<Dish, AppError> {
    let dish = sqlx::query_as!(
        Dish,
        r#"INSERT INTO dishes (french, japanese, reading, genre, notes)
           VALUES (?, ?, ?, ?, ?)
           RETURNING id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at"#,
        new.french, new.japanese, new.reading, new.genre, new.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(dish)
}

pub async fn update_dish(pool: &SqlitePool, id: i64, update: UpdateDish) -> Result<Dish, AppError> {
    let existing = get_dish(pool, id).await?;
    let french   = update.french.unwrap_or(existing.french);
    let japanese = update.japanese.unwrap_or(existing.japanese);
    let reading  = update.reading.or(existing.reading);
    let genre    = update.genre.or(existing.genre);
    let notes    = update.notes.or(existing.notes);

    let dish = sqlx::query_as!(
        Dish,
        r#"UPDATE dishes SET french=?, japanese=?, reading=?, genre=?, notes=?
           WHERE id=?
           RETURNING id as "id!", french, japanese, reading, genre as "genre: _", notes, created_at"#,
        french, japanese, reading, genre, notes, id
    )
    .fetch_one(pool)
    .await?;
    Ok(dish)
}

pub async fn delete_dish(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM dishes WHERE id = ?", id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
