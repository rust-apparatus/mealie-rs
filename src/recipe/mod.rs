use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Tag {
    id: uuid::Uuid,
    name: String,
    slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredient {
    quantity: f64,
    unit: Option<String>,
    food: Option<String>,
    note: String,
    is_food: bool,
    disable_amount: bool,
    display: String,
    title: Option<String>,
    original_text: Option<String>,
    reference_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInstruction {
    id: uuid::Uuid,
    title: String,
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    household_id: uuid::Uuid,
    group_id: uuid::Uuid,
    name: String,
    slug: String,
    image: String,
    recipe_yield: String,
    total_time: String,
    prep_time: String,
    cook_time: Option<String>,
    perform_time: String,
    description: String,
    tags: Vec<Tag>,
    #[serde(rename = "orgURL")]
    org_url: String,
    recipe_ingredient: Vec<RecipeIngredient>,
    recipe_instructions: Vec<RecipeInstruction>,
}

#[derive(thiserror::Error, Debug)]
pub enum RecipeError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type RecipeResult<T> = Result<T, RecipeError>;

impl Recipe {
    pub fn from_json_file(path: &str) -> RecipeResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let recipe: Self = serde_json::from_reader(reader)?;
        Ok(recipe)
    }
}
