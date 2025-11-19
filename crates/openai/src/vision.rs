use crate::{OpenAIClient, OpenAIError, ModelConfig, OpenAIModel};
use crate::client::ChatMessage;
use models::Recipe;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Vision API client for image processing
#[derive(Clone)]
pub struct VisionClient {
    client: OpenAIClient,
}

impl VisionClient {
    pub fn new(client: OpenAIClient) -> Self {
        Self { client }
    }

    /// Extract recipe from image (OCR + parsing)
    pub async fn extract_recipe_from_image(
        &self,
        image_url: &str,
        detail: ImageDetail,
        model: OpenAIModel,
    ) -> Result<Recipe, OpenAIError> {
        let schema = self.recipe_json_schema();

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a recipe extraction expert. Extract the complete recipe from the provided image, including all ingredients with quantities and units, and all steps in order.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!(
                    r#"[{{"type": "image_url", "image_url": {{"url": "{}", "detail": "{}"}}}}]"#,
                    image_url,
                    detail.as_str()
                ),
            },
        ];

        let config = ModelConfig::new(model).with_temperature(0.3);
        let result = self.client.structured_output(messages, schema, config).await?;

        let recipe: Recipe = serde_json::from_value(result)
            .map_err(|e| OpenAIError::SerializationError(e.to_string()))?;

        Ok(recipe)
    }

    /// Detect pantry items from image
    pub async fn detect_pantry_items(
        &self,
        image_url: &str,
        detail: ImageDetail,
        model: OpenAIModel,
    ) -> Result<Vec<String>, OpenAIError> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a food recognition expert. Identify all food items visible in the image and return them as a simple list.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!(
                    r#"[{{"type": "image_url", "image_url": {{"url": "{}", "detail": "{}"}}}}]"#,
                    image_url,
                    detail.as_str()
                ),
            },
        ];

        let config = ModelConfig::new(model).with_temperature(0.3);
        let response = self.client.chat_completion(messages, config).await?;

        let content = response.choices.first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| OpenAIError::InvalidResponse("No content in response".to_string()))?;

        // Parse list from response (simple line-by-line parsing)
        let items: Vec<String> = content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    // Remove bullet points, numbers, etc.
                    Some(trimmed.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == '*').trim().to_string())
                }
            })
            .collect();

        Ok(items)
    }

    /// Analyze dish image for inspiration
    pub async fn analyze_dish_image(
        &self,
        image_url: &str,
        detail: ImageDetail,
        model: OpenAIModel,
    ) -> Result<DishAnalysis, OpenAIError> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a culinary expert. Analyze the dish in the image and provide insights about the cuisine, ingredients, techniques, and potential recipe.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!(
                    r#"[{{"type": "image_url", "image_url": {{"url": "{}", "detail": "{}"}}}}]"#,
                    image_url,
                    detail.as_str()
                ),
            },
        ];

        let config = ModelConfig::new(model).with_temperature(0.7);
        let response = self.client.chat_completion(messages, config).await?;

        let content = response.choices.first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| OpenAIError::InvalidResponse("No content in response".to_string()))?;

        Ok(DishAnalysis {
            description: content.clone(),
            suggested_ingredients: vec![],
            cuisine_type: None,
        })
    }

    /// Recipe JSON schema for structured output
    fn recipe_json_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "required": ["title", "servings", "ingredients", "steps"],
            "properties": {
                "title": {"type": "string"},
                "servings": {"type": "integer", "minimum": 1},
                "description": {"type": "string"},
                "ingredients": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["name", "quantity", "unit"],
                        "properties": {
                            "name": {"type": "string"},
                            "quantity": {"type": "number"},
                            "unit": {"type": "string"},
                            "notes": {"type": "string"}
                        }
                    }
                },
                "steps": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["text"],
                        "properties": {
                            "text": {"type": "string"},
                            "timer_sec": {"type": "integer"},
                            "tools": {"type": "array", "items": {"type": "string"}}
                        }
                    }
                }
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Low,
    High,
    Auto,
}

impl ImageDetail {
    pub fn as_str(&self) -> &str {
        match self {
            ImageDetail::Low => "low",
            ImageDetail::High => "high",
            ImageDetail::Auto => "auto",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DishAnalysis {
    pub description: String,
    pub suggested_ingredients: Vec<String>,
    pub cuisine_type: Option<String>,
}
