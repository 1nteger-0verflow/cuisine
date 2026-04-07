use sqlx::SqlitePool;

use crate::{
    db::relations,
    error::AppError,
    models::relation::TermCategory,
    models::technique::{NewTechnique, Technique, TechniqueDetail, TechniqueQuery, UpdateTechnique},
};

pub async fn list_techniques(
    pool: &SqlitePool,
    query: &TechniqueQuery,
) -> Result<Vec<Technique>, AppError> {
    let techniques = if let Some(q) = &query.q {
        let pattern = format!("%{}%", q);
        sqlx::query_as!(
            Technique,
            r#"SELECT id as "id!", french, reading, notes, created_at
               FROM techniques
               WHERE french LIKE ? OR notes LIKE ?
               ORDER BY french"#,
            pattern, pattern
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            Technique,
            r#"SELECT id as "id!", french, reading, notes, created_at
               FROM techniques ORDER BY french"#
        )
        .fetch_all(pool)
        .await?
    };
    Ok(techniques)
}

pub async fn get_technique(pool: &SqlitePool, id: i64) -> Result<Technique, AppError> {
    let technique: Option<Technique> = sqlx::query_as!(
        Technique,
        r#"SELECT id as "id!", french, reading, notes, created_at
           FROM techniques WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    technique.ok_or(AppError::NotFound)
}

pub async fn get_technique_detail(
    pool: &SqlitePool,
    id: i64,
) -> Result<TechniqueDetail, AppError> {
    let technique = get_technique(pool, id).await?;
    let related_terms = relations::get_related_terms(pool, &TermCategory::Technique, id).await?;
    Ok(TechniqueDetail { technique, related_terms })
}

pub async fn create_technique(pool: &SqlitePool, new: NewTechnique) -> Result<Technique, AppError> {
    let technique = sqlx::query_as!(
        Technique,
        r#"INSERT INTO techniques (french, reading, notes)
           VALUES (?, ?, ?)
           RETURNING id as "id!", french, reading, notes, created_at"#,
        new.french, new.reading, new.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(technique)
}

pub async fn update_technique(
    pool: &SqlitePool,
    id: i64,
    update: UpdateTechnique,
) -> Result<Technique, AppError> {
    let existing = get_technique(pool, id).await?;
    let french  = update.french.unwrap_or(existing.french);
    let reading = update.reading.or(existing.reading);
    let notes   = update.notes.or(existing.notes);

    let technique = sqlx::query_as!(
        Technique,
        r#"UPDATE techniques SET french=?, reading=?, notes=?
           WHERE id=?
           RETURNING id as "id!", french, reading, notes, created_at"#,
        french, reading, notes, id
    )
    .fetch_one(pool)
    .await?;
    Ok(technique)
}

pub async fn delete_technique(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM techniques WHERE id = ?", id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
