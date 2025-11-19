use models::{User, UserProfile, UpdatePreferencesRequest};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create(&self, email: &str, password_hash: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        // Create default profile
        sqlx::query!(
            "INSERT INTO user_profiles (user_id) VALUES ($1)",
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    /// Get user by ID
    pub async fn get(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Get user by email
    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Get user profile
    pub async fn get_profile(&self, user_id: Uuid) -> Result<Option<UserProfile>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, dietary_restrictions, preferred_cuisines, health_goals,
                   budget_preference, pantry_items, preferences, updated_at
            FROM user_profiles
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| UserProfile {
            user_id: r.user_id,
            dietary_restrictions: r.dietary_restrictions.as_array()
                .and_then(|arr| serde_json::from_value(serde_json::Value::Array(arr.clone())).ok())
                .unwrap_or_default(),
            preferred_cuisines: r.preferred_cuisines.as_array()
                .and_then(|arr| serde_json::from_value(serde_json::Value::Array(arr.clone())).ok())
                .unwrap_or_default(),
            health_goals: r.health_goals.as_array()
                .and_then(|arr| serde_json::from_value(serde_json::Value::Array(arr.clone())).ok())
                .unwrap_or_default(),
            budget_preference: r.budget_preference
                .and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok()),
            pantry_items: r.pantry_items.as_array()
                .and_then(|arr| serde_json::from_value(serde_json::Value::Array(arr.clone())).ok())
                .unwrap_or_default(),
            preferences: r.preferences,
            updated_at: r.updated_at,
        }))
    }

    /// Update user preferences
    pub async fn update_preferences(
        &self,
        user_id: Uuid,
        request: UpdatePreferencesRequest,
    ) -> Result<UserProfile, sqlx::Error> {
        let mut query = String::from("UPDATE user_profiles SET ");
        let mut updates = Vec::new();
        let mut param_count = 1;

        if let Some(dietary) = &request.dietary_restrictions {
            updates.push(format!("dietary_restrictions = ${}", param_count));
            param_count += 1;
        }
        if let Some(cuisines) = &request.preferred_cuisines {
            updates.push(format!("preferred_cuisines = ${}", param_count));
            param_count += 1;
        }
        if let Some(goals) = &request.health_goals {
            updates.push(format!("health_goals = ${}", param_count));
            param_count += 1;
        }
        if let Some(_budget) = &request.budget_preference {
            updates.push(format!("budget_preference = ${}", param_count));
            param_count += 1;
        }
        if let Some(pantry) = &request.pantry_items {
            updates.push(format!("pantry_items = ${}", param_count));
            param_count += 1;
        }
        if let Some(_prefs) = &request.preferences {
            updates.push(format!("preferences = ${}", param_count));
            param_count += 1;
        }

        if updates.is_empty() {
            // No updates, just return current profile
            return self.get_profile(user_id).await?.ok_or(sqlx::Error::RowNotFound);
        }

        query.push_str(&updates.join(", "));
        query.push_str(&format!(" WHERE user_id = ${}", param_count));

        let mut query_builder = sqlx::query(&query);

        if let Some(dietary) = &request.dietary_restrictions {
            query_builder = query_builder.bind(serde_json::to_value(dietary).unwrap());
        }
        if let Some(cuisines) = &request.preferred_cuisines {
            query_builder = query_builder.bind(serde_json::to_value(cuisines).unwrap());
        }
        if let Some(goals) = &request.health_goals {
            query_builder = query_builder.bind(serde_json::to_value(goals).unwrap());
        }
        if let Some(budget) = &request.budget_preference {
            query_builder = query_builder.bind(serde_json::to_value(budget).unwrap());
        }
        if let Some(pantry) = &request.pantry_items {
            query_builder = query_builder.bind(serde_json::to_value(pantry).unwrap());
        }
        if let Some(prefs) = &request.preferences {
            query_builder = query_builder.bind(prefs);
        }
        query_builder = query_builder.bind(user_id);

        query_builder.execute(&self.pool).await?;

        self.get_profile(user_id).await?.ok_or(sqlx::Error::RowNotFound)
    }
}
