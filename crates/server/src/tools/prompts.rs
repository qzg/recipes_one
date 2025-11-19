use crate::state::AppState;
use openai::realtime::{SessionUpdate, ToolDefinition};
use serde_json::json;

/// Inject privileged system prompts and tool definitions
pub async fn inject_privileged_prompt(
    state: &AppState,
    session_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let instructions = r#"
You are an expert culinary AI assistant for a voice-first recipe app. Your role is to:

1. Help users create, modify, and improve recipes through natural conversation
2. Provide cooking guidance, timing, and technique advice
3. Suggest cost-optimized or enriched versions of recipes
4. Generate creative recipe variants (improvise, cultural twists, healthy versions)
5. Assist with ingredient recognition from images
6. Help users publish beautiful recipe cards and social media content

Key principles:
- Always maintain the canonical recipe state through structured tool calls
- Use apply_recipe_patch for any recipe modifications
- Suggest UI chips via create_ui_nudges for next-step interactions
- Keep responses conversational and helpful
- Validate all recipe changes before applying
- Log user preferences when learned through persist_user_prefs

Available capabilities:
- Recipe management (get, update, merge)
- Cost analysis and optimization
- Recipe remixing (improvise, cultural, healthy)
- Image analysis (OCR recipes, detect pantry items)
- Publishing (cards, social media)
- Analytics tracking

When the user uploads an image, automatically detect if it contains a recipe or ingredients and offer appropriate actions.
"#;

    let tools = get_tool_definitions();

    // In a real implementation, this would be sent via OpenAI's webhook/session update mechanism
    // For now, we'll log it
    tracing::info!("Injecting privileged prompt for session: {}", session_id);
    tracing::debug!("Tools count: {}", tools.len());

    Ok(())
}

fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        // Recipe tools
        ToolDefinition::new(
            "get_recipe",
            "Fetch the canonical recipe by ID",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"}
                },
                "required": ["recipe_id"]
            }),
        ),
        ToolDefinition::new(
            "apply_recipe_patch",
            "Apply updates to a recipe with optimistic concurrency control",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "expected_version": {"type": "integer"},
                    "update": {"type": "object"}
                },
                "required": ["recipe_id", "expected_version", "update"]
            }),
        ),
        ToolDefinition::new(
            "create_ui_nudges",
            "Generate UI chip suggestions for next actions",
            json!({
                "type": "object",
                "properties": {
                    "context": {"type": "string"},
                    "recipe_id": {"type": "string", "format": "uuid"}
                }
            }),
        ),
        ToolDefinition::new(
            "merge_recipe",
            "Combine multiple recipe variants into a new recipe",
            json!({
                "type": "object",
                "properties": {
                    "recipe_ids": {
                        "type": "array",
                        "items": {"type": "string", "format": "uuid"}
                    }
                },
                "required": ["recipe_ids"]
            }),
        ),
        ToolDefinition::new(
            "persist_user_prefs",
            "Save learned user preferences",
            json!({
                "type": "object",
                "properties": {
                    "preferences": {"type": "object"}
                },
                "required": ["preferences"]
            }),
        ),
        ToolDefinition::new(
            "generate_shopping_list",
            "Generate shopping list for recipe",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "pantry_items": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                },
                "required": ["recipe_id"]
            }),
        ),

        // Image tools
        ToolDefinition::new(
            "extract_recipe_from_image",
            "Extract recipe from image using OCR and parsing",
            json!({
                "type": "object",
                "properties": {
                    "image_url": {"type": "string"},
                    "detail": {"type": "string", "enum": ["low", "high", "auto"]},
                    "model": {"type": "string"}
                },
                "required": ["image_url"]
            }),
        ),
        ToolDefinition::new(
            "detect_pantry_items",
            "Recognize ingredients from image",
            json!({
                "type": "object",
                "properties": {
                    "image_url": {"type": "string"},
                    "detail": {"type": "string", "enum": ["low", "high", "auto"]}
                },
                "required": ["image_url"]
            }),
        ),

        // Cost tools
        ToolDefinition::new(
            "pricing_compute",
            "Compute estimated cost for recipe",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "region": {"type": "string"},
                    "store": {"type": "string"},
                    "source": {"type": "string", "enum": ["model_based", "walmart_api", "amazon_api"]}
                },
                "required": ["recipe_id", "recipe_version", "region", "source"]
            }),
        ),
        ToolDefinition::new(
            "cost_cutter",
            "Suggest lower-cost recipe version",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "strategies": {
                        "type": "array",
                        "items": {"type": "string", "enum": ["off_brand", "bulk_size", "alternate_protein", "seasonal", "substitution"]}
                    }
                },
                "required": ["recipe_id", "recipe_version", "strategies"]
            }),
        ),
        ToolDefinition::new(
            "enrich_recipe",
            "Upgrade recipe with premium ingredients and techniques",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "fields": {
                        "type": "array",
                        "items": {"type": "string", "enum": ["premium_ingredients", "advanced_techniques", "plating_instructions", "wine_pairing", "garnishes"]}
                    }
                },
                "required": ["recipe_id", "recipe_version", "fields"]
            }),
        ),

        // Remix tools
        ToolDefinition::new(
            "remix_improvise",
            "Create recipe variant using pantry items",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "pantry_items": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "budget_cap": {"type": "integer"}
                },
                "required": ["recipe_id", "recipe_version", "pantry_items"]
            }),
        ),
        ToolDefinition::new(
            "remix_cultural_twist",
            "Adapt recipe to different cuisine",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "region": {"type": "string"}
                },
                "required": ["recipe_id", "recipe_version", "region"]
            }),
        ),
        ToolDefinition::new(
            "remix_healthy_mode",
            "Create healthier version of recipe",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "user_targets": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                },
                "required": ["recipe_id", "recipe_version"]
            }),
        ),

        // Publisher tools
        ToolDefinition::new(
            "publish_recipe_card",
            "Generate recipe card image/PDF",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "format": {"type": "string", "enum": ["png", "pdf", "html", "social_square", "social_story", "social_landscape"]},
                    "theme_id": {"type": "string", "format": "uuid"}
                },
                "required": ["recipe_id", "recipe_version", "format"]
            }),
        ),
        ToolDefinition::new(
            "publish_social_media",
            "Generate social media posts and captions",
            json!({
                "type": "object",
                "properties": {
                    "recipe_id": {"type": "string", "format": "uuid"},
                    "recipe_version": {"type": "integer"},
                    "platforms": {
                        "type": "array",
                        "items": {"type": "string", "enum": ["instagram", "tiktok", "pinterest", "twitter", "facebook", "youtube"]}
                    },
                    "theme_id": {"type": "string", "format": "uuid"}
                },
                "required": ["recipe_id", "recipe_version", "platforms"]
            }),
        ),
    ]
}
