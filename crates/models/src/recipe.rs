use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Recipe image with role and metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeImage {
    pub role: ImageRole,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    pub source: ImageSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_step: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_ingredient: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageRole {
    Cover,
    Finished,
    InProgress,
    Ingredient,
    Step,
    Thumbnail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageSource {
    User,
    Ai,
    Uploaded,
    Generated,
}

/// Recipe ingredient with optional image reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
}

/// Recipe step with optional timer, tools, and image
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Step {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
}

/// Core recipe structure (canonical state)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    pub id: Uuid,
    pub version: i32,
    pub title: String,
    pub servings: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<RecipeImage>,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,

    // Metadata (not in JSON schema but useful for DB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Recipe {
    pub fn new(title: String, servings: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            version: 1,
            title,
            servings,
            description: None,
            images: Vec::new(),
            ingredients: Vec::new(),
            steps: Vec::new(),
            user_id: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Validate recipe structure
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.servings == 0 {
            return Err("Servings must be greater than 0".to_string());
        }
        if self.ingredients.is_empty() {
            return Err("Recipe must have at least one ingredient".to_string());
        }
        if self.steps.is_empty() {
            return Err("Recipe must have at least one step".to_string());
        }
        Ok(())
    }
}

/// Recipe patch for optimistic updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipePatch {
    pub recipe_id: Uuid,
    pub expected_version: i32,
    pub patch: serde_json::Value, // JSON Patch format
}

/// Request to apply a recipe patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyPatchRequest {
    pub recipe_id: Uuid,
    pub expected_version: i32,
    #[serde(flatten)]
    pub update: RecipeUpdate,
}

/// Flexible recipe update (can be patch or full replacement)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecipeUpdate {
    FullRecipe(Recipe),
    PartialUpdate {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        servings: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        images: Option<Vec<RecipeImage>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ingredients: Option<Vec<Ingredient>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        steps: Option<Vec<Step>>,
    },
}

/// Shopping list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingListItem {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub in_pantry: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_recipe() {
        let recipe = Recipe::new("Test Recipe".to_string(), 4);
        assert_eq!(recipe.title, "Test Recipe");
        assert_eq!(recipe.servings, 4);
        assert_eq!(recipe.version, 1);
    }

    #[test]
    fn test_recipe_validation() {
        let mut recipe = Recipe::new("Test".to_string(), 4);

        // Should fail - no ingredients or steps
        assert!(recipe.validate().is_err());

        recipe.ingredients.push(Ingredient {
            name: "Flour".to_string(),
            quantity: 2.0,
            unit: "cups".to_string(),
            notes: None,
            image_ref: None,
        });

        recipe.steps.push(Step {
            text: "Mix ingredients".to_string(),
            timer_sec: None,
            tools: vec![],
            image_ref: None,
        });

        // Should pass now
        assert!(recipe.validate().is_ok());
    }

    #[test]
    fn test_recipe_serialization() {
        let recipe = Recipe::new("Test Recipe".to_string(), 4);
        let json = serde_json::to_string(&recipe).unwrap();
        let deserialized: Recipe = serde_json::from_str(&json).unwrap();
        assert_eq!(recipe.title, deserialized.title);
    }
}
