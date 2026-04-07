use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::relation::RelatedTermRef;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum IngredientGenre {
    Dairy,
    Herb,
    Spice,
    Vegetable,
    Mushroom,
    Protein,
    Grain,
    Seafood,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: i64,
    pub french: String,
    pub reading: Option<String>,
    pub genre: Option<IngredientGenre>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngredientDetail {
    #[serde(flatten)]
    pub ingredient: Ingredient,
    pub related_terms: Vec<RelatedTermRef>,
}

#[derive(Debug, Deserialize)]
pub struct NewIngredient {
    pub french: String,
    pub reading: Option<String>,
    pub genre: Option<IngredientGenre>,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateIngredient {
    pub french: Option<String>,
    pub reading: Option<String>,
    pub genre: Option<IngredientGenre>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IngredientQuery {
    pub genre: Option<IngredientGenre>,
    pub q: Option<String>,
}
