use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Dish,
    Ingredient,
    Utensil,
    Technique,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Term {
    pub id: i64,
    pub french: String,
    pub japanese: String,
    pub category: Category,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewTerm {
    pub french: String,
    pub japanese: String,
    pub category: Category,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateTerm {
    pub french: Option<String>,
    pub japanese: Option<String>,
    pub category: Option<Category>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TermQuery {
    pub category: Option<Category>,
    pub q: Option<String>,
}
