use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Analytics event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    ViewRecipe,
    ViewCard,
    ShareSocial,
    ClickBuy,
    BuySuccess,
    CreateRecipe,
    UpdateRecipe,
    DeleteRecipe,
    UseTool,
    ChipClick,
}

/// Generic analytics event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub recipe_id: Option<Uuid>,
    pub event_type: EventType,
    pub platform: Option<String>, // "instagram", "tiktok", "pinterest", etc.
    pub metadata: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
}

impl AnalyticsEvent {
    pub fn new(event_type: EventType, user_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            recipe_id: None,
            event_type,
            platform: None,
            metadata: serde_json::json!({}),
            timestamp: chrono::Utc::now(),
            session_id: None,
        }
    }

    pub fn with_recipe(mut self, recipe_id: Uuid) -> Self {
        self.recipe_id = Some(recipe_id);
        self
    }

    pub fn with_platform(mut self, platform: String) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Conversion tracking (purchases, monetization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversion {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recipe_id: Uuid,
    pub revenue_cents: i32,
    pub platform: String, // Where the conversion happened
    pub conversion_type: ConversionType,
    pub metadata: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConversionType {
    RecipePurchase,      // Direct recipe sale
    IngredientPurchase,  // Affiliate link click
    SubscriptionSignup,  // Premium subscription
    AdRevenue,           // Ad impression/click
}

/// Request to log an analytics event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEventRequest {
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Request to record a conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordConversionRequest {
    pub recipe_id: Uuid,
    pub revenue_cents: i32,
    pub platform: String,
    pub conversion_type: ConversionType,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Analytics summary for a recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeAnalytics {
    pub recipe_id: Uuid,
    pub views: i64,
    pub shares: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub revenue_cents: i64,
    pub conversion_rate: f64, // clicks -> conversions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_event_builder() {
        let user_id = Uuid::new_v4();
        let recipe_id = Uuid::new_v4();

        let event = AnalyticsEvent::new(EventType::ViewRecipe, Some(user_id))
            .with_recipe(recipe_id)
            .with_platform("web".to_string());

        assert_eq!(event.event_type, EventType::ViewRecipe);
        assert_eq!(event.user_id, Some(user_id));
        assert_eq!(event.recipe_id, Some(recipe_id));
        assert_eq!(event.platform, Some("web".to_string()));
    }
}
