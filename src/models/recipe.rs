use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: i64,
    pub name_french: String,
    pub description_japanese: Option<String>,
    pub difficulty: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeDetail {
    #[serde(flatten)]
    pub recipe: Recipe,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeIngredient {
    pub term_id: i64,
    pub french: String,
    pub japanese: String,
    pub quantity: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeStep {
    pub id: i64,
    pub step_number: i64,
    pub instruction_french: Option<String>,
    pub instruction_japanese: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewRecipe {
    pub name_french: String,
    pub description_japanese: Option<String>,
    pub difficulty: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateRecipe {
    pub name_french: Option<String>,
    pub description_japanese: Option<String>,
    pub difficulty: Option<String>,
}
