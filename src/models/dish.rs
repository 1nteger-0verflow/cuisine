use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::relation::RelatedTermRef;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DishGenre {
    Soup,
    Stew,
    Dessert,
    Pastry,
    Main,
    Appetizer,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Dish {
    pub id: i64,
    pub french: String,
    pub japanese: String,
    pub reading: Option<String>,
    pub genre: Option<DishGenre>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DishDetail {
    #[serde(flatten)]
    pub dish: Dish,
    pub related_terms: Vec<RelatedTermRef>,
}

#[derive(Debug, Deserialize)]
pub struct NewDish {
    pub french: String,
    pub japanese: String,
    pub reading: Option<String>,
    pub genre: Option<DishGenre>,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateDish {
    pub french: Option<String>,
    pub japanese: Option<String>,
    pub reading: Option<String>,
    pub genre: Option<DishGenre>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DishQuery {
    pub genre: Option<DishGenre>,
    pub q: Option<String>,
}
