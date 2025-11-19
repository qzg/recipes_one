use crate::state::AppState;
use models::{CostAnalysis, CostSource, IngredientCost, Recipe};
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;

pub async fn pricing_compute(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;
    let recipe_version: i32 = serde_json::from_value(args["recipe_version"].clone())?;
    let region: String = serde_json::from_value(args["region"].clone())?;
    let store: Option<String> = serde_json::from_value(args.get("store").cloned().unwrap_or(Value::Null))?;
    let source: CostSource = serde_json::from_value(args["source"].clone())?;

    let recipe = state.db.recipes().get(recipe_id).await?;

    // Phase 1: Model-based pricing estimation
    let breakdown = estimate_ingredient_costs(&recipe, &region, &source).await?;

    let total_cost_cents: i32 = breakdown.iter().map(|b| b.cost_cents).sum();
    let cost_per_serving_cents = total_cost_cents / recipe.servings as i32;

    let analysis = CostAnalysis {
        recipe_id,
        recipe_version,
        total_cost_cents,
        cost_per_serving_cents,
        region,
        store,
        source,
        breakdown,
        computed_at: Utc::now(),
    };

    // Save analysis
    state.db.cost().save_analysis(&analysis).await?;

    Ok(serde_json::to_value(analysis)?)
}

pub async fn cost_cutter(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;
    let recipe_version: i32 = serde_json::from_value(args["recipe_version"].clone())?;
    let strategies: Vec<String> = serde_json::from_value(args["strategies"].clone())?;

    let recipe = state.db.recipes().get(recipe_id).await?;

    // Generate cost-optimized variant
    let mut optimized = recipe.clone();
    optimized.id = Uuid::new_v4();
    optimized.version = 1;
    optimized.title = format!("{} (Budget-Friendly)", recipe.title);
    optimized.description = Some(format!(
        "Cost-optimized version using strategies: {}",
        strategies.join(", ")
    ));

    // Apply cost-cutting strategies (simplified)
    for strategy in &strategies {
        match strategy.as_str() {
            "off_brand" => {
                // Note in description that generic brands are recommended
                optimized.description = Some(format!(
                    "{}\n\nTip: Use store-brand or generic versions where available.",
                    optimized.description.as_deref().unwrap_or("")
                ));
            }
            "bulk_size" => {
                optimized.description = Some(format!(
                    "{}\n\nTip: Buy in bulk and freeze extras to reduce per-serving cost.",
                    optimized.description.as_deref().unwrap_or("")
                ));
            }
            "alternate_protein" => {
                // Could swap expensive proteins with cheaper alternatives
                // For now, just note it
                optimized.description = Some(format!(
                    "{}\n\nConsider: Chicken thighs instead of breasts, ground turkey instead of beef.",
                    optimized.description.as_deref().unwrap_or("")
                ));
            }
            _ => {}
        }
    }

    Ok(serde_json::to_value(optimized)?)
}

pub async fn enrich_recipe(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;
    let recipe_version: i32 = serde_json::from_value(args["recipe_version"].clone())?;
    let fields: Vec<String> = serde_json::from_value(args["fields"].clone())?;

    let recipe = state.db.recipes().get(recipe_id).await?;

    let mut enriched = recipe.clone();
    enriched.id = Uuid::new_v4();
    enriched.version = 1;
    enriched.title = format!("{} (Premium)", recipe.title);

    // Apply enrichments (simplified)
    for field in &fields {
        match field.as_str() {
            "premium_ingredients" => {
                enriched.description = Some(format!(
                    "{}\n\nUpgrade: Use premium quality ingredients - organic produce, artisanal cheese, grass-fed meat.",
                    enriched.description.as_deref().unwrap_or("")
                ));
            }
            "advanced_techniques" => {
                enriched.description = Some(format!(
                    "{}\n\nTechniques: Consider sous vide, precise temperature control, or advanced knife skills.",
                    enriched.description.as_deref().unwrap_or("")
                ));
            }
            "plating_instructions" => {
                enriched.description = Some(format!(
                    "{}\n\nPlating: Arrange thoughtfully on warm plates, use height, add color contrast.",
                    enriched.description.as_deref().unwrap_or("")
                ));
            }
            "wine_pairing" => {
                enriched.description = Some(format!(
                    "{}\n\nWine Pairing: Recommended wine pairing suggestions based on flavor profile.",
                    enriched.description.as_deref().unwrap_or("")
                ));
            }
            _ => {}
        }
    }

    Ok(serde_json::to_value(enriched)?)
}

// Helper: Estimate ingredient costs (Phase 1: model-based)
async fn estimate_ingredient_costs(
    recipe: &Recipe,
    region: &str,
    source: &CostSource,
) -> Result<Vec<IngredientCost>, Box<dyn std::error::Error + Send + Sync>> {
    let mut breakdown = vec![];

    for ingredient in &recipe.ingredients {
        // Simple heuristic pricing (would be replaced with model calls or API in production)
        let base_price = estimate_base_price(&ingredient.name, ingredient.quantity, &ingredient.unit);

        breakdown.push(IngredientCost {
            ingredient_name: ingredient.name.clone(),
            quantity: ingredient.quantity,
            unit: ingredient.unit.clone(),
            cost_cents: base_price,
            source: source.clone(),
            product_url: None,
            brand: None,
        });
    }

    Ok(breakdown)
}

fn estimate_base_price(name: &str, quantity: f64, unit: &str) -> i32 {
    // Very simple heuristic - would be replaced with AI model or API
    let name_lower = name.to_lowercase();

    let base = if name_lower.contains("chicken") || name_lower.contains("beef") || name_lower.contains("pork") {
        500 // $5 per lb
    } else if name_lower.contains("fish") || name_lower.contains("salmon") {
        800 // $8 per lb
    } else if name_lower.contains("cheese") {
        400 // $4 per lb
    } else if name_lower.contains("vegetable") || name_lower.contains("tomato") || name_lower.contains("onion") {
        200 // $2 per lb
    } else if name_lower.contains("spice") || name_lower.contains("salt") || name_lower.contains("pepper") {
        50 // $0.50
    } else {
        150 // $1.50 default
    };

    // Scale by quantity (rough approximation)
    let multiplier = match unit {
        "lb" | "lbs" | "pound" | "pounds" => quantity,
        "oz" | "ounce" | "ounces" => quantity / 16.0,
        "cup" | "cups" => quantity * 0.5,
        "tbsp" | "tablespoon" | "tablespoons" => quantity * 0.03,
        "tsp" | "teaspoon" | "teaspoons" => quantity * 0.01,
        _ => quantity * 0.25,
    };

    ((base as f64) * multiplier) as i32
}
