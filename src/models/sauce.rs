use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::relation::RelatedTermRef;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SauceGenre {
    Mere,
    Derivee,
    Froide,
    Emulsionnee,
    Beurre,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sauce {
    pub id: i64,
    pub french: String,
    pub reading: Option<String>,
    pub genre: Option<SauceGenre>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SauceDetail {
    #[serde(flatten)]
    pub sauce: Sauce,
    pub related_terms: Vec<RelatedTermRef>,
}

#[derive(Debug, Deserialize)]
pub struct NewSauce {
    pub french: String,
    pub reading: Option<String>,
    pub genre: Option<SauceGenre>,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateSauce {
    pub french: Option<String>,
    pub reading: Option<String>,
    pub genre: Option<SauceGenre>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SauceQuery {
    pub genre: Option<SauceGenre>,
    pub q: Option<String>,
}
