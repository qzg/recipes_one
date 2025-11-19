use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Realtime session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub openai_session_id: Option<String>,
    pub active_recipe_id: Option<Uuid>,
    pub status: SessionStatus,
    pub ephemeral_token: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Expired,
    Terminated,
}

/// Request to create an ephemeral session token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_id: Option<Uuid>,
}

/// Response with ephemeral token for OpenAI Realtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub session_id: Uuid,
    pub ephemeral_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// OpenAI Realtime webhook event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeWebhookEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub event_id: String,
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

/// Tool call from OpenAI Realtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub call_id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Tool output to return to OpenAI Realtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub call_id: String,
    pub output: serde_json::Value,
}

/// UI nudges/chips for next-step suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UINudge {
    pub id: String,
    pub label: String,
    pub action: NudgeAction,
    pub priority: i32, // Higher = more prominent
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NudgeAction {
    EstimateCost,
    TryCostCutter,
    EnrichRecipe,
    GenerateVariant,
    CulturalTwist,
    HealthyMode,
    PublishCard,
    ShareSocial,
    AddShoppingList,
    RecognizeImage,
}

/// Remix variant generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemixRequest {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub mode: RemixMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pantry_items: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_cap: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RemixMode {
    Improvise,
    CulturalTwist,
    HealthyMode,
}

/// Remix run history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemixRun {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source_recipe_id: Uuid,
    pub source_version: i32,
    pub result_recipe_id: Option<Uuid>,
    pub mode: RemixMode,
    pub parameters: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = RealtimeSession {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            openai_session_id: None,
            active_recipe_id: None,
            status: SessionStatus::Active,
            ephemeral_token: None,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            last_activity: chrono::Utc::now(),
        };

        assert_eq!(session.status, SessionStatus::Active);
    }

    #[test]
    fn test_ui_nudge() {
        let nudge = UINudge {
            id: "cost-estimate".to_string(),
            label: "Estimate cost".to_string(),
            action: NudgeAction::EstimateCost,
            priority: 10,
        };

        assert_eq!(nudge.action, NudgeAction::EstimateCost);
    }
}
