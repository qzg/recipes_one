use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User profile with preferences (diet, cuisine, health, budget)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: Uuid,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dietary_restrictions: Vec<String>, // e.g., "vegetarian", "gluten-free", "dairy-free"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_cuisines: Vec<String>, // e.g., "italian", "mexican", "japanese"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub health_goals: Vec<String>, // e.g., "low-carb", "high-protein", "low-sodium"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_preference: Option<BudgetPreference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pantry_items: Vec<String>, // User's available ingredients
    #[serde(default)]
    pub preferences: serde_json::Value, // Flexible key-value store for learned preferences
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPreference {
    Minimal,
    Budget,
    Moderate,
    Premium,
    Luxury,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            user_id: Uuid::nil(),
            dietary_restrictions: Vec::new(),
            preferred_cuisines: Vec::new(),
            health_goals: Vec::new(),
            budget_preference: None,
            pantry_items: Vec::new(),
            preferences: serde_json::json!({}),
            updated_at: chrono::Utc::now(),
        }
    }
}

/// Request to update user preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePreferencesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dietary_restrictions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_cuisines: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_goals: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_preference: Option<BudgetPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pantry_items: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<serde_json::Value>,
}

/// JWT claims for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, expiration_hours: i64) -> Self {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::hours(expiration_hours)).timestamp() as usize;
        Self {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp() as usize,
        }
    }
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response with JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

/// Public user info (no password hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile_default() {
        let profile = UserProfile::default();
        assert!(profile.dietary_restrictions.is_empty());
        assert!(profile.preferred_cuisines.is_empty());
    }

    #[test]
    fn test_claims_creation() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id, 24);
        assert_eq!(claims.sub, user_id.to_string());
        assert!(claims.exp > claims.iat);
    }
}
