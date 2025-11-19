use models::{AnalyticsEvent, Conversion, EventType, ConversionType, RecipeAnalytics};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AnalyticsRepository {
    pool: PgPool,
}

impl AnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Log an analytics event
    pub async fn log_event(&self, event: &AnalyticsEvent) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO analytics_events (id, user_id, recipe_id, session_id, event_type, platform, metadata, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            event.id,
            event.user_id,
            event.recipe_id,
            event.session_id,
            serde_json::to_string(&event.event_type).unwrap_or_default().trim_matches('"'),
            event.platform,
            event.metadata,
            event.timestamp
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Record a conversion
    pub async fn record_conversion(&self, conversion: &Conversion) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO conversions (id, user_id, recipe_id, revenue_cents, platform, conversion_type, metadata, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            conversion.id,
            conversion.user_id,
            conversion.recipe_id,
            conversion.revenue_cents,
            conversion.platform,
            serde_json::to_string(&conversion.conversion_type).unwrap_or_default().trim_matches('"'),
            conversion.metadata,
            conversion.timestamp
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get analytics summary for a recipe
    pub async fn get_recipe_analytics(&self, recipe_id: Uuid) -> Result<RecipeAnalytics, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(CASE WHEN event_type = 'view_recipe' THEN 1 END) as views,
                COUNT(CASE WHEN event_type = 'share_social' THEN 1 END) as shares,
                COUNT(CASE WHEN event_type = 'click_buy' THEN 1 END) as clicks
            FROM analytics_events
            WHERE recipe_id = $1
            "#,
            recipe_id
        )
        .fetch_one(&self.pool)
        .await?;

        let conversion_row = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as conversions,
                COALESCE(SUM(revenue_cents), 0) as revenue
            FROM conversions
            WHERE recipe_id = $1
            "#,
            recipe_id
        )
        .fetch_one(&self.pool)
        .await?;

        let views = row.views.unwrap_or(0);
        let shares = row.shares.unwrap_or(0);
        let clicks = row.clicks.unwrap_or(0);
        let conversions = conversion_row.conversions.unwrap_or(0);
        let revenue = conversion_row.revenue.unwrap_or(0);

        let conversion_rate = if clicks > 0 {
            conversions as f64 / clicks as f64
        } else {
            0.0
        };

        Ok(RecipeAnalytics {
            recipe_id,
            views,
            shares,
            clicks,
            conversions,
            revenue_cents: revenue,
            conversion_rate,
        })
    }

    /// Get user activity summary
    pub async fn get_user_activity(&self, user_id: Uuid, event_type: Option<EventType>) -> Result<Vec<AnalyticsEvent>, sqlx::Error> {
        let events = if let Some(et) = event_type {
            let event_type_str = serde_json::to_string(&et).unwrap_or_default().trim_matches('"').to_string();
            sqlx::query!(
                r#"
                SELECT id, user_id, recipe_id, session_id, event_type, platform, metadata, timestamp
                FROM analytics_events
                WHERE user_id = $1 AND event_type = $2
                ORDER BY timestamp DESC
                LIMIT 100
                "#,
                user_id,
                event_type_str
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                SELECT id, user_id, recipe_id, session_id, event_type, platform, metadata, timestamp
                FROM analytics_events
                WHERE user_id = $1
                ORDER BY timestamp DESC
                LIMIT 100
                "#,
                user_id
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(events.into_iter().map(|row| {
            AnalyticsEvent {
                id: row.id,
                user_id: row.user_id,
                recipe_id: row.recipe_id,
                event_type: serde_json::from_str(&format!("\"{}\"", row.event_type)).unwrap_or(EventType::ViewRecipe),
                platform: row.platform,
                metadata: row.metadata,
                timestamp: row.timestamp,
                session_id: row.session_id,
            }
        }).collect())
    }
}
