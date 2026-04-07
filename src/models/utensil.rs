use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::relation::RelatedTermRef;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Utensil {
    pub id: i64,
    pub french: String,
    pub reading: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UtensilDetail {
    #[serde(flatten)]
    pub utensil: Utensil,
    pub related_terms: Vec<RelatedTermRef>,
}

#[derive(Debug, Deserialize)]
pub struct NewUtensil {
    pub french: String,
    pub reading: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateUtensil {
    pub french: Option<String>,
    pub reading: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UtensilQuery {
    pub q: Option<String>,
}
