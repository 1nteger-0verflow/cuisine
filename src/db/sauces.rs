use sqlx::SqlitePool;

use crate::{
    db::relations,
    error::AppError,
    models::sauce::{NewSauce, Sauce, SauceDetail, SauceQuery, UpdateSauce},
    models::relation::TermCategory,
};

pub async fn list_sauces(pool: &SqlitePool, query: &SauceQuery) -> Result<Vec<Sauce>, AppError> {
    let sauces = match (&query.genre, &query.q) {
        (Some(g), Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Sauce,
                r#"SELECT id as "id!", french, reading, genre as "genre: _", notes, created_at
                   FROM sauces
                   WHERE genre = ? AND (french LIKE ? OR notes LIKE ?)
                   ORDER BY french"#,
                g, pattern, pattern
            )
            .fetch_all(pool)
            .await?
        }
        (Some(g), None) => {
            sqlx::query_as!(
                Sauce,
                r#"SELECT id as "id!", french, reading, genre as "genre: _", notes, created_at
                   FROM sauces WHERE genre = ? ORDER BY french"#,
                g
            )
            .fetch_all(pool)
            .await?
        }
        (None, Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Sauce,
                r#"SELECT id as "id!", french, reading, genre as "genre: _", notes, created_at
                   FROM sauces
                   WHERE french LIKE ? OR notes LIKE ?
                   ORDER BY french"#,
                pattern, pattern
            )
            .fetch_all(pool)
            .await?
        }
        (None, None) => {
            sqlx::query_as!(
                Sauce,
                r#"SELECT id as "id!", french, reading, genre as "genre: _", notes, created_at
                   FROM sauces ORDER BY french"#
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(sauces)
}

pub async fn get_sauce(pool: &SqlitePool, id: i64) -> Result<Sauce, AppError> {
    let sauce: Option<Sauce> = sqlx::query_as!(
        Sauce,
        r#"SELECT id as "id!", french, reading, genre as "genre: _", notes, created_at
           FROM sauces WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    sauce.ok_or(AppError::NotFound)
}

pub async fn get_sauce_detail(pool: &SqlitePool, id: i64) -> Result<SauceDetail, AppError> {
    let sauce = get_sauce(pool, id).await?;
    let related_terms = relations::get_related_terms(pool, &TermCategory::Sauce, id).await?;
    Ok(SauceDetail { sauce, related_terms })
}

pub async fn create_sauce(pool: &SqlitePool, new: NewSauce) -> Result<Sauce, AppError> {
    let sauce = sqlx::query_as!(
        Sauce,
        r#"INSERT INTO sauces (french, reading, genre, notes)
           VALUES (?, ?, ?, ?)
           RETURNING id as "id!", french, reading, genre as "genre: _", notes, created_at"#,
        new.french, new.reading, new.genre, new.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(sauce)
}

pub async fn update_sauce(
    pool: &SqlitePool,
    id: i64,
    update: UpdateSauce,
) -> Result<Sauce, AppError> {
    let existing = get_sauce(pool, id).await?;
    let french  = update.french.unwrap_or(existing.french);
    let reading = update.reading.or(existing.reading);
    let genre   = update.genre.or(existing.genre);
    let notes   = update.notes.or(existing.notes);

    let sauce = sqlx::query_as!(
        Sauce,
        r#"UPDATE sauces SET french=?, reading=?, genre=?, notes=?
           WHERE id=?
           RETURNING id as "id!", french, reading, genre as "genre: _", notes, created_at"#,
        french, reading, genre, notes, id
    )
    .fetch_one(pool)
    .await?;
    Ok(sauce)
}

pub async fn delete_sauce(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM sauces WHERE id = ?", id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
