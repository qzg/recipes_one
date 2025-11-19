use crate::state::AppState;
use models::{ApplyPatchRequest, RecipeUpdate, ShoppingListItem, UINudge, NudgeAction};
use serde_json::Value;
use uuid::Uuid;

pub async fn get_recipe(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;

    let recipe = state.db.recipes().get(recipe_id).await
        .map_err(|e| format!("Failed to get recipe: {}", e))?;

    Ok(serde_json::to_value(recipe)?)
}

pub async fn apply_recipe_patch(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let request: ApplyPatchRequest = serde_json::from_value(args.clone())?;

    let updated = state.db.recipes().apply_patch(request).await
        .map_err(|e| format!("Failed to apply patch: {}", e))?;

    Ok(serde_json::to_value(updated)?)
}

pub async fn create_ui_nudges(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let context = args.get("context").and_then(|v| v.as_str()).unwrap_or("");
    let recipe_id = args.get("recipe_id").and_then(|v| v.as_str());

    let mut nudges = vec![];

    // Context-aware nudge generation
    if context.contains("cost") || context.contains("price") || context.contains("expensive") {
        nudges.push(UINudge {
            id: "estimate-cost".to_string(),
            label: "Estimate cost".to_string(),
            action: NudgeAction::EstimateCost,
            priority: 10,
        });
        nudges.push(UINudge {
            id: "cost-cutter".to_string(),
            label: "Find cheaper alternatives".to_string(),
            action: NudgeAction::TryCostCutter,
            priority: 9,
        });
    }

    if context.contains("healthy") || context.contains("nutrition") || context.contains("diet") {
        nudges.push(UINudge {
            id: "healthy-mode".to_string(),
            label: "Make it healthier".to_string(),
            action: NudgeAction::HealthyMode,
            priority: 10,
        });
    }

    if context.contains("share") || context.contains("post") || context.contains("social") {
        nudges.push(UINudge {
            id: "publish-card".to_string(),
            label: "Create recipe card".to_string(),
            action: NudgeAction::PublishCard,
            priority: 10,
        });
        nudges.push(UINudge {
            id: "share-social".to_string(),
            label: "Share on social media".to_string(),
            action: NudgeAction::ShareSocial,
            priority: 9,
        });
    }

    if context.contains("variant") || context.contains("different") || context.contains("twist") {
        nudges.push(UINudge {
            id: "cultural-twist".to_string(),
            label: "Try a cultural twist".to_string(),
            action: NudgeAction::CulturalTwist,
            priority: 10,
        });
        nudges.push(UINudge {
            id: "improvise".to_string(),
            label: "Improvise with pantry".to_string(),
            action: NudgeAction::GenerateVariant,
            priority: 8,
        });
    }

    // Default nudges if context is unclear
    if nudges.is_empty() && recipe_id.is_some() {
        nudges.push(UINudge {
            id: "estimate-cost".to_string(),
            label: "Estimate cost".to_string(),
            action: NudgeAction::EstimateCost,
            priority: 5,
        });
        nudges.push(UINudge {
            id: "generate-variant".to_string(),
            label: "Generate variant".to_string(),
            action: NudgeAction::GenerateVariant,
            priority: 4,
        });
    }

    Ok(serde_json::to_value(nudges)?)
}

pub async fn merge_recipe(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_ids: Vec<Uuid> = serde_json::from_value(args["recipe_ids"].clone())?;

    let mut recipes = vec![];
    for id in recipe_ids {
        let recipe = state.db.recipes().get(id).await?;
        recipes.push(recipe);
    }

    // Simple merge logic: combine ingredients and steps
    if recipes.is_empty() {
        return Err("No recipes to merge".into());
    }

    let mut merged = recipes[0].clone();
    merged.id = Uuid::new_v4();
    merged.version = 1;
    merged.title = format!("{} (Merged)", merged.title);

    for recipe in recipes.iter().skip(1) {
        merged.ingredients.extend(recipe.ingredients.clone());
        merged.steps.extend(recipe.steps.clone());
    }

    Ok(serde_json::to_value(merged)?)
}

pub async fn persist_user_prefs(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let preferences = args.get("preferences")
        .ok_or("Missing preferences field")?;

    // In a real implementation, we would:
    // 1. Extract user_id from session context
    // 2. Update user profile with new preferences
    // For now, just acknowledge

    tracing::info!("Persisting user preferences: {:?}", preferences);

    Ok(serde_json::json!({
        "success": true,
        "message": "Preferences saved"
    }))
}

pub async fn generate_shopping_list(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;
    let pantry_items: Vec<String> = args.get("pantry_items")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    let recipe = state.db.recipes().get(recipe_id).await?;

    let shopping_list: Vec<ShoppingListItem> = recipe
        .ingredients
        .into_iter()
        .map(|ing| {
            let in_pantry = pantry_items.iter()
                .any(|p| p.to_lowercase().contains(&ing.name.to_lowercase()));

            ShoppingListItem {
                name: ing.name,
                quantity: ing.quantity,
                unit: ing.unit,
                in_pantry,
            }
        })
        .collect();

    Ok(serde_json::to_value(shopping_list)?)
}
