use sqlx::SqlitePool;

use crate::{
    error::AppError,
    models::relation::{NewRelation, RelatedTermRef, TermCategory},
};

pub async fn get_related_terms(
    pool: &SqlitePool,
    category: &TermCategory,
    id: i64,
) -> Result<Vec<RelatedTermRef>, AppError> {
    let cat = category_str(category);
    let related = sqlx::query_as!(
        RelatedTermRef,
        r#"SELECT refs.other_cat  AS "category: _",
                  refs.other_id   AS "id!",
                  COALESCE(d.french,  i.french,  u.french,  tq.french)   AS "french!",
                  refs.relation_type
           FROM (
             SELECT to_category   AS other_cat, to_id   AS other_id, relation_type
               FROM term_relations WHERE from_category = ? AND from_id = ?
             UNION ALL
             SELECT from_category AS other_cat, from_id AS other_id, relation_type
               FROM term_relations WHERE to_category = ? AND to_id = ?
           ) refs
           LEFT JOIN dishes      d  ON refs.other_cat = 'dish'       AND d.id  = refs.other_id
           LEFT JOIN ingredients i  ON refs.other_cat = 'ingredient' AND i.id  = refs.other_id
           LEFT JOIN sauces      s  ON refs.other_cat = 'sauce'      AND s.id  = refs.other_id
           LEFT JOIN utensils    u  ON refs.other_cat = 'utensil'    AND u.id  = refs.other_id
           LEFT JOIN techniques  tq ON refs.other_cat = 'technique'  AND tq.id = refs.other_id
           ORDER BY COALESCE(d.french, i.french, s.french, u.french, tq.french)"#,
        cat, id, cat, id
    )
    .fetch_all(pool)
    .await?;
    Ok(related)
}

pub async fn add_relation(
    pool: &SqlitePool,
    from_category: &TermCategory,
    from_id: i64,
    relation: NewRelation,
) -> Result<(), AppError> {
    let from_cat = category_str(from_category);
    let to_cat = category_str(&relation.to_category);
    let to_id = relation.to_id;
    let relation_type = relation.relation_type.unwrap_or_else(|| "related".to_string());
    sqlx::query!(
        "INSERT INTO term_relations (from_category, from_id, to_category, to_id, relation_type)
         VALUES (?, ?, ?, ?, ?)",
        from_cat, from_id, to_cat, to_id, relation_type
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_relation(
    pool: &SqlitePool,
    from_category: &TermCategory,
    from_id: i64,
    to_category: &TermCategory,
    to_id: i64,
) -> Result<(), AppError> {
    let fc = category_str(from_category);
    let tc = category_str(to_category);
    let result = sqlx::query!(
        "DELETE FROM term_relations
         WHERE (from_category=? AND from_id=? AND to_category=? AND to_id=?)
            OR (from_category=? AND from_id=? AND to_category=? AND to_id=?)",
        fc, from_id, tc, to_id,
        tc, to_id,  fc, from_id
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub fn category_str(cat: &TermCategory) -> &'static str {
    match cat {
        TermCategory::Dish       => "dish",
        TermCategory::Ingredient => "ingredient",
        TermCategory::Sauce      => "sauce",
        TermCategory::Utensil    => "utensil",
        TermCategory::Technique  => "technique",
    }
}
