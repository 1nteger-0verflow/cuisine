use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TermCategory {
    Dish,
    Ingredient,
    Sauce,
    Utensil,
    Technique,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RelatedTermRef {
    pub category: TermCategory,
    pub id: i64,
    pub french: String,
    pub relation_type: String,
}

#[derive(Debug, Deserialize)]
pub struct NewRelation {
    pub to_category: TermCategory,
    pub to_id: i64,
    pub relation_type: Option<String>,
}
