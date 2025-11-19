use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Cost analysis result for a recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnalysis {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub total_cost_cents: i32,
    pub cost_per_serving_cents: i32,
    pub region: String,
    pub store: Option<String>,
    pub source: CostSource,
    pub breakdown: Vec<IngredientCost>,
    pub computed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CostSource {
    ModelBased,     // Phase 1: AI-predicted pricing
    WalmartApi,     // Phase 2: Real API
    AmazonApi,      // Phase 2: Real API
    ManualEntry,    // User-provided
}

/// Cost breakdown for a single ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientCost {
    pub ingredient_name: String,
    pub quantity: f64,
    pub unit: String,
    pub cost_cents: i32,
    pub source: CostSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
}

/// Cost optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CostStrategy {
    OffBrand,       // Use generic brands
    BulkSize,       // Buy in bulk
    AlternateProtein, // Swap expensive proteins
    Seasonal,       // Use seasonal ingredients
    Substitution,   // Generic ingredient swaps
}

/// Request to compute recipe cost
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCostRequest {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    pub source: CostSource,
}

/// Request to optimize recipe cost
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCutterRequest {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub strategies: Vec<CostStrategy>,
}

/// Request to enrich/upgrade recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichRecipeRequest {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub fields: Vec<EnrichmentField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EnrichmentField {
    PremiumIngredients,
    AdvancedTechniques,
    PlatingInstructions,
    WinePairing,
    Garnishes,
}

/// Cached pricing data for the cost catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingCatalogEntry {
    pub id: Uuid,
    pub ingredient_name: String,
    pub region: String,
    pub store: Option<String>,
    pub price_cents: i32,
    pub quantity: f64,
    pub unit: String,
    pub source: CostSource,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_analysis() {
        let analysis = CostAnalysis {
            recipe_id: Uuid::new_v4(),
            recipe_version: 1,
            total_cost_cents: 1250,
            cost_per_serving_cents: 312,
            region: "US-CA".to_string(),
            store: Some("Walmart".to_string()),
            source: CostSource::ModelBased,
            breakdown: vec![],
            computed_at: chrono::Utc::now(),
        };

        assert_eq!(analysis.total_cost_cents, 1250);
        assert_eq!(analysis.cost_per_serving_cents, 312);
    }
}
