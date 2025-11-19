use models::{RealtimeSession, SessionStatus};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new session
    pub async fn create(
        &self,
        user_id: Uuid,
        active_recipe_id: Option<Uuid>,
        expires_at: chrono::DateTime<Utc>,
    ) -> Result<RealtimeSession, sqlx::Error> {
        let session = sqlx::query_as!(
            RealtimeSession,
            r#"
            INSERT INTO sessions (user_id, active_recipe_id, status, expires_at)
            VALUES ($1, $2, 'active', $3)
            RETURNING id, user_id, openai_session_id, active_recipe_id,
                      status as "status: SessionStatus", ephemeral_token,
                      created_at, expires_at, last_activity
            "#,
            user_id,
            active_recipe_id,
            expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    /// Get session by ID
    pub async fn get(&self, session_id: Uuid) -> Result<Option<RealtimeSession>, sqlx::Error> {
        sqlx::query_as!(
            RealtimeSession,
            r#"
            SELECT id, user_id, openai_session_id, active_recipe_id,
                   status as "status: SessionStatus", ephemeral_token,
                   created_at, expires_at, last_activity
            FROM sessions
            WHERE id = $1
            "#,
            session_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Update session with OpenAI details
    pub async fn update_openai_session(
        &self,
        session_id: Uuid,
        openai_session_id: &str,
        ephemeral_token: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET openai_session_id = $1, ephemeral_token = $2 WHERE id = $3",
            openai_session_id,
            ephemeral_token,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update last activity timestamp
    pub async fn update_activity(&self, session_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET last_activity = NOW() WHERE id = $1",
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Terminate a session
    pub async fn terminate(&self, session_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET status = 'terminated' WHERE id = $1",
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE sessions SET status = 'expired' WHERE expires_at < NOW() AND status = 'active'"
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
