use sqlx::SqlitePool;

use crate::{
    error::AppError,
    models::term::{NewTerm, Term, TermQuery, UpdateTerm},
};

pub async fn list_terms(pool: &SqlitePool, query: &TermQuery) -> Result<Vec<Term>, AppError> {
    let terms = match (&query.category, &query.q) {
        (Some(cat), Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Term,
                r#"SELECT id as "id!", french, japanese, category as "category: _", notes, created_at
                   FROM terms
                   WHERE category = ? AND (french LIKE ? OR japanese LIKE ?)
                   ORDER BY french"#,
                cat,
                pattern,
                pattern
            )
            .fetch_all(pool)
            .await?
        }
        (Some(cat), None) => {
            sqlx::query_as!(
                Term,
                r#"SELECT id as "id!", french, japanese, category as "category: _", notes, created_at
                   FROM terms
                   WHERE category = ?
                   ORDER BY french"#,
                cat
            )
            .fetch_all(pool)
            .await?
        }
        (None, Some(q)) => {
            let pattern = format!("%{}%", q);
            sqlx::query_as!(
                Term,
                r#"SELECT id as "id!", french, japanese, category as "category: _", notes, created_at
                   FROM terms
                   WHERE french LIKE ? OR japanese LIKE ?
                   ORDER BY french"#,
                pattern,
                pattern
            )
            .fetch_all(pool)
            .await?
        }
        (None, None) => {
            sqlx::query_as!(
                Term,
                r#"SELECT id as "id!", french, japanese, category as "category: _", notes, created_at
                   FROM terms
                   ORDER BY french"#
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(terms)
}

pub async fn get_term(pool: &SqlitePool, id: i64) -> Result<Term, AppError> {
    let term: Option<Term> = sqlx::query_as!(
        Term,
        r#"SELECT id as "id!", french, japanese, category as "category: _", notes, created_at
           FROM terms WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    term.ok_or(AppError::NotFound)
}

pub async fn create_term(pool: &SqlitePool, new: NewTerm) -> Result<Term, AppError> {
    let term = sqlx::query_as!(
        Term,
        r#"INSERT INTO terms (french, japanese, category, notes)
           VALUES (?, ?, ?, ?)
           RETURNING id as "id!", french, japanese, category as "category: _", notes, created_at"#,
        new.french,
        new.japanese,
        new.category,
        new.notes
    )
    .fetch_one(pool)
    .await?;
    Ok(term)
}

pub async fn update_term(pool: &SqlitePool, id: i64, update: UpdateTerm) -> Result<Term, AppError> {
    let existing = get_term(pool, id).await?;
    let french = update.french.unwrap_or(existing.french);
    let japanese = update.japanese.unwrap_or(existing.japanese);
    let category = update.category.unwrap_or(existing.category);
    let notes = update.notes.or(existing.notes);

    let term = sqlx::query_as!(
        Term,
        r#"UPDATE terms
           SET french = ?, japanese = ?, category = ?, notes = ?
           WHERE id = ?
           RETURNING id as "id!", french, japanese, category as "category: _", notes, created_at"#,
        french,
        japanese,
        category,
        notes,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(term)
}

pub async fn delete_term(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result: sqlx::sqlite::SqliteQueryResult =
        sqlx::query!("DELETE FROM terms WHERE id = ?", id).execute(pool).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::term::{Category, NewTerm, UpdateTerm};

    fn new_term(french: &str, japanese: &str, category: Category) -> NewTerm {
        NewTerm { french: french.to_string(), japanese: japanese.to_string(), category, notes: None }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_term(pool: SqlitePool) {
        let term = create_term(&pool, new_term("beurre", "バター", Category::Ingredient))
            .await
            .unwrap();
        assert_eq!(term.french, "beurre");
        assert_eq!(term.japanese, "バター");
        assert_eq!(term.category, Category::Ingredient);
        assert!(term.id > 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_term(pool: SqlitePool) {
        let created = create_term(&pool, new_term("beurre", "バター", Category::Ingredient))
            .await
            .unwrap();
        let fetched = get_term(&pool, created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.japanese, "バター");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_term_not_found(pool: SqlitePool) {
        let err = get_term(&pool, 999).await.unwrap_err();
        assert!(matches!(err, AppError::NotFound));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_terms_by_category(pool: SqlitePool) {
        create_term(&pool, new_term("beurre", "バター", Category::Ingredient)).await.unwrap();
        create_term(&pool, new_term("bouillabaisse", "ブイヤベース", Category::Dish)).await.unwrap();

        let query = TermQuery { category: Some(Category::Ingredient), q: None };
        let results = list_terms(&pool, &query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].french, "beurre");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_terms_search(pool: SqlitePool) {
        create_term(&pool, new_term("beurre noisette", "ブール・ノワゼット", Category::Technique))
            .await
            .unwrap();
        create_term(&pool, new_term("beurre blanc", "ブール・ブラン", Category::Technique))
            .await
            .unwrap();

        let query = TermQuery { category: None, q: Some("noisette".to_string()) };
        let results = list_terms(&pool, &query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].french, "beurre noisette");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_term_partial(pool: SqlitePool) {
        let created = create_term(&pool, new_term("beurre", "バター", Category::Ingredient))
            .await
            .unwrap();

        let update = UpdateTerm { japanese: Some("澄ましバター".to_string()), ..Default::default() };
        let updated = update_term(&pool, created.id, update).await.unwrap();
        assert_eq!(updated.french, "beurre");
        assert_eq!(updated.japanese, "澄ましバター");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_term(pool: SqlitePool) {
        let created = create_term(&pool, new_term("beurre", "バター", Category::Ingredient))
            .await
            .unwrap();
        delete_term(&pool, created.id).await.unwrap();
        let err = get_term(&pool, created.id).await.unwrap_err();
        assert!(matches!(err, AppError::NotFound));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_term_not_found(pool: SqlitePool) {
        let err = delete_term(&pool, 999).await.unwrap_err();
        assert!(matches!(err, AppError::NotFound));
    }
}
