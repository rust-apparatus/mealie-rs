use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Tag {
    id: uuid::Uuid,
    name: String,
    slug: String,
}
impl Tag {
    pub fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_slug(&self) -> &str {
        &self.slug
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
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
impl RecipeIngredient {
    pub fn get_quantity(&self) -> f64 {
        self.quantity
    }
    pub fn get_unit(&self) -> &Option<String> {
        &self.unit
    }
    pub fn get_food(&self) -> &Option<String> {
        &self.food
    }
    pub fn get_note(&self) -> &str {
        &self.note
    }
    pub fn get_is_food(&self) -> bool {
        self.is_food
    }
    pub fn get_disable_amount(&self) -> bool {
        self.disable_amount
    }
    pub fn get_display(&self) -> &str {
        &self.display
    }
    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }
    pub fn get_original_text(&self) -> &Option<String> {
        &self.original_text
    }
    pub fn get_reference_id(&self) -> &Option<uuid::Uuid> {
        &self.reference_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
pub struct RecipeInstruction {
    id: uuid::Uuid,
    title: String,
    text: String,
}
impl RecipeInstruction {
    pub fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Recipe {
    id: uuid::Uuid,
    name: String,
    slug: String,
    image: String,
    recipe_yield: Option<String>,
    total_time: Option<String>,
    prep_time: Option<String>,
    cook_time: Option<String>,
    perform_time: Option<String>,
    description: Option<String>,
    tags: Vec<Tag>,
    org_url: Option<String>,
    recipe_ingredient: Vec<RecipeIngredient>,
    recipe_instructions: Vec<RecipeInstruction>,
}
impl Recipe {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_slug(&self) -> &str {
        &self.slug
    }
    pub fn get_image(&self) -> &str {
        &self.image
    }
    pub fn get_recipe_yield(&self) -> Option<String> {
        self.recipe_yield.clone()
    }
    pub fn get_total_time(&self) -> Option<String> {
        self.total_time.clone()
    }
    pub fn get_prep_time(&self) -> Option<String> {
        self.prep_time.clone()
    }
    pub fn get_cook_time(&self) -> Option<String> {
        self.cook_time.clone()
    }
    pub fn get_perform_time(&self) -> Option<String> {
        self.perform_time.clone()
    }
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }
    pub fn get_org_url(&self) -> Option<String> {
        self.org_url.clone()
    }
    pub fn get_recipe_ingredient(&self) -> &Vec<RecipeIngredient> {
        &self.recipe_ingredient
    }
    pub fn get_recipe_instructions(&self) -> &Vec<RecipeInstruction> {
        &self.recipe_instructions
    }
    pub fn from_json_file(path: &str) -> RecipeResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let recipe: Self = serde_json::from_reader(reader)?;
        Ok(recipe)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RecipeError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type RecipeResult<T> = Result<T, RecipeError>;
