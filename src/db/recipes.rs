use sqlx::SqlitePool;

use crate::{
    error::AppError,
    models::recipe::{NewRecipe, Recipe, RecipeDetail, RecipeIngredient, RecipeStep, UpdateRecipe},
};

pub async fn list_recipes(pool: &SqlitePool) -> Result<Vec<Recipe>, AppError> {
    let recipes = sqlx::query_as!(
        Recipe,
        r#"SELECT id as "id!", name_french, description_japanese, difficulty, created_at
           FROM recipes
           ORDER BY name_french"#
    )
    .fetch_all(pool)
    .await?;
    Ok(recipes)
}

pub async fn get_recipe(pool: &SqlitePool, id: i64) -> Result<Recipe, AppError> {
    let recipe: Option<Recipe> = sqlx::query_as!(
        Recipe,
        r#"SELECT id as "id!", name_french, description_japanese, difficulty, created_at
           FROM recipes WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    recipe.ok_or(AppError::NotFound)
}

pub async fn get_recipe_detail(pool: &SqlitePool, id: i64) -> Result<RecipeDetail, AppError> {
    let recipe = get_recipe(pool, id).await?;

    let ingredients = sqlx::query_as!(
        RecipeIngredient,
        r#"SELECT t.id as "term_id!", t.french, t.japanese, ri.quantity, ri.notes
           FROM recipe_ingredients ri
           JOIN terms t ON t.id = ri.term_id
           WHERE ri.recipe_id = ?
           ORDER BY t.french"#,
        id
    )
    .fetch_all(pool)
    .await?;

    let steps = sqlx::query_as!(
        RecipeStep,
        r#"SELECT id as "id!", step_number as "step_number!", instruction_french, instruction_japanese
           FROM recipe_steps
           WHERE recipe_id = ?
           ORDER BY step_number"#,
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(RecipeDetail { recipe, ingredients, steps })
}

pub async fn create_recipe(pool: &SqlitePool, new: NewRecipe) -> Result<Recipe, AppError> {
    let recipe = sqlx::query_as!(
        Recipe,
        r#"INSERT INTO recipes (name_french, description_japanese, difficulty)
           VALUES (?, ?, ?)
           RETURNING id as "id!", name_french, description_japanese, difficulty, created_at"#,
        new.name_french,
        new.description_japanese,
        new.difficulty
    )
    .fetch_one(pool)
    .await?;
    Ok(recipe)
}

pub async fn update_recipe(
    pool: &SqlitePool,
    id: i64,
    update: UpdateRecipe,
) -> Result<Recipe, AppError> {
    let existing = get_recipe(pool, id).await?;
    let name_french = update.name_french.unwrap_or(existing.name_french);
    let description_japanese = update.description_japanese.or(existing.description_japanese);
    let difficulty = update.difficulty.or(existing.difficulty);

    let recipe = sqlx::query_as!(
        Recipe,
        r#"UPDATE recipes
           SET name_french = ?, description_japanese = ?, difficulty = ?
           WHERE id = ?
           RETURNING id as "id!", name_french, description_japanese, difficulty, created_at"#,
        name_french,
        description_japanese,
        difficulty,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(recipe)
}

pub async fn delete_recipe(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result: sqlx::sqlite::SqliteQueryResult =
        sqlx::query!("DELETE FROM recipes WHERE id = ?", id).execute(pool).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::terms::create_term,
        models::{
            recipe::{NewRecipe, UpdateRecipe},
            term::NewTerm,
        },
    };

    fn new_recipe(name: &str) -> NewRecipe {
        NewRecipe { name_french: name.to_string(), description_japanese: None, difficulty: None }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_recipe(pool: SqlitePool) {
        let recipe = create_recipe(&pool, new_recipe("bouillabaisse")).await.unwrap();
        assert_eq!(recipe.name_french, "bouillabaisse");
        assert!(recipe.id > 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_recipe_not_found(pool: SqlitePool) {
        let err = get_recipe(&pool, 999).await.unwrap_err();
        assert!(matches!(err, AppError::NotFound));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_recipe_detail_empty(pool: SqlitePool) {
        let recipe = create_recipe(&pool, new_recipe("bouillabaisse")).await.unwrap();
        let detail = get_recipe_detail(&pool, recipe.id).await.unwrap();
        assert_eq!(detail.recipe.id, recipe.id);
        assert!(detail.ingredients.is_empty());
        assert!(detail.steps.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_recipe_detail_with_ingredients(pool: SqlitePool) {
        let recipe = create_recipe(&pool, new_recipe("bouillabaisse")).await.unwrap();
        let term = create_term(
            &pool,
            NewTerm {
                french: "safran".to_string(),
                japanese: "サフラン".to_string(),
                category: "ingredient".to_string(),
                notes: None,
            },
        )
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO recipe_ingredients (recipe_id, term_id, quantity) VALUES (?, ?, ?)",
            recipe.id,
            term.id,
            "1 pinch"
        )
        .execute(&pool)
        .await
        .unwrap();

        let detail = get_recipe_detail(&pool, recipe.id).await.unwrap();
        assert_eq!(detail.ingredients.len(), 1);
        assert_eq!(detail.ingredients[0].french, "safran");
        assert_eq!(detail.ingredients[0].japanese, "サフラン");
        assert_eq!(detail.ingredients[0].quantity.as_deref(), Some("1 pinch"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_recipe_partial(pool: SqlitePool) {
        let recipe = create_recipe(
            &pool,
            NewRecipe {
                name_french: "bouillabaisse".to_string(),
                description_japanese: None,
                difficulty: Some("hard".to_string()),
            },
        )
        .await
        .unwrap();

        let update = UpdateRecipe {
            description_japanese: Some("マルセイユの魚介スープ".to_string()),
            ..Default::default()
        };
        let updated = update_recipe(&pool, recipe.id, update).await.unwrap();
        assert_eq!(updated.name_french, "bouillabaisse");
        assert_eq!(updated.description_japanese.as_deref(), Some("マルセイユの魚介スープ"));
        assert_eq!(updated.difficulty.as_deref(), Some("hard"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_recipe_cascades(pool: SqlitePool) {
        let recipe = create_recipe(&pool, new_recipe("bouillabaisse")).await.unwrap();
        let term = create_term(
            &pool,
            NewTerm {
                french: "safran".to_string(),
                japanese: "サフラン".to_string(),
                category: "ingredient".to_string(),
                notes: None,
            },
        )
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO recipe_ingredients (recipe_id, term_id) VALUES (?, ?)",
            recipe.id,
            term.id
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO recipe_steps (recipe_id, step_number, instruction_french) VALUES (?, ?, ?)",
            recipe.id,
            1,
            "Faites mijoter."
        )
        .execute(&pool)
        .await
        .unwrap();

        delete_recipe(&pool, recipe.id).await.unwrap();

        let ing_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM recipe_ingredients WHERE recipe_id = ?",
            recipe.id
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        let step_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM recipe_steps WHERE recipe_id = ?",
            recipe.id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(ing_count, 0);
        assert_eq!(step_count, 0);
    }
}
