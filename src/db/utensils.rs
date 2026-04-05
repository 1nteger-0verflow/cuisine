use sqlx::SqlitePool;

use crate::{
    db::relations,
    error::AppError,
    models::relation::TermCategory,
    models::utensil::{NewUtensil, UpdateUtensil, Utensil, UtensilDetail, UtensilQuery},
};

pub async fn list_utensils(
    pool: &SqlitePool,
    query: &UtensilQuery,
) -> Result<Vec<Utensil>, AppError> {
    let utensils = if let Some(q) = &query.q {
        let pattern = format!("%{}%", q);
        sqlx::query_as!(
            Utensil,
            r#"SELECT id as "id!", french, japanese, reading, notes, created_at
               FROM utensils
               WHERE french LIKE ? OR japanese LIKE ?
               ORDER BY french"#,
            pattern, pattern
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            Utensil,
            r#"SELECT id as "id!", french, japanese, reading, notes, created_at
               FROM utensils ORDER BY french"#
        )
        .fetch_all(pool)
        .await?
    };
    Ok(utensils)
}

pub async fn get_utensil(pool: &SqlitePool, id: i64) -> Result<Utensil, AppError> {
    let utensil: Option<Utensil> = sqlx::query_as!(
        Utensil,
        r#"SELECT id as "id!", french, japanese, reading, notes, created_at
           FROM utensils WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    utensil.ok_or(AppError::NotFound)
}

pub async fn get_utensil_detail(pool: &SqlitePool, id: i64) -> Result<UtensilDetail, AppError> {
    let utensil = get_utensil(pool, id).await?;
    let related_terms = relations::get_related_terms(pool, &TermCategory::Utensil, id).await?;
    Ok(UtensilDetail { utensil, related_terms })
}

pub async fn create_utensil(pool: &SqlitePool, new: NewUtensil) -> Result<Utensil, AppError> {
    let utensil = sqlx::query_as!(
        Utensil,
        r#"INSERT INTO utensils (french, japanese, reading, notes)
           VALUES (?, ?, ?, ?)
           RETURNING id as "id!", french, japanese, reading, notes, created_at"#,
        new.french, new.japanese, new.reading, new.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(utensil)
}

pub async fn update_utensil(
    pool: &SqlitePool,
    id: i64,
    update: UpdateUtensil,
) -> Result<Utensil, AppError> {
    let existing = get_utensil(pool, id).await?;
    let french   = update.french.unwrap_or(existing.french);
    let japanese = update.japanese.unwrap_or(existing.japanese);
    let reading  = update.reading.or(existing.reading);
    let notes    = update.notes.or(existing.notes);

    let utensil = sqlx::query_as!(
        Utensil,
        r#"UPDATE utensils SET french=?, japanese=?, reading=?, notes=?
           WHERE id=?
           RETURNING id as "id!", french, japanese, reading, notes, created_at"#,
        french, japanese, reading, notes, id
    )
    .fetch_one(pool)
    .await?;
    Ok(utensil)
}

pub async fn delete_utensil(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM utensils WHERE id = ?", id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
