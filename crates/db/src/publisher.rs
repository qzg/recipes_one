use models::{PublishJob, PublishJobType, JobStatus, BrandTheme, GeneratedAsset};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PublisherRepository {
    pool: PgPool,
}

impl PublisherRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a publish job
    pub async fn create_job(
        &self,
        user_id: Uuid,
        recipe_id: Uuid,
        recipe_version: i32,
        job_type: PublishJobType,
    ) -> Result<PublishJob, sqlx::Error> {
        let job_type_str = serde_json::to_string(&job_type)
            .map_err(|e| sqlx::Error::Encode(Box::new(e)))?
            .trim_matches('"').to_string();

        let job = sqlx::query!(
            r#"
            INSERT INTO publish_jobs (user_id, recipe_id, recipe_version, job_type, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, recipe_id, recipe_version, job_type, status, result, error, created_at, completed_at
            "#,
            user_id,
            recipe_id,
            recipe_version,
            job_type_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(PublishJob {
            id: job.id,
            user_id: job.user_id,
            recipe_id: job.recipe_id,
            recipe_version: job.recipe_version,
            job_type: serde_json::from_str(&format!("\"{}\"", job.job_type)).unwrap_or(PublishJobType::RecipeCard),
            status: serde_json::from_str(&format!("\"{}\"", job.status)).unwrap_or(JobStatus::Pending),
            result: job.result,
            error: job.error,
            created_at: job.created_at,
            completed_at: job.completed_at,
        })
    }

    /// Update job status
    pub async fn update_job_status(
        &self,
        job_id: Uuid,
        status: JobStatus,
        result: Option<serde_json::Value>,
        error: Option<String>,
    ) -> Result<(), sqlx::Error> {
        let status_str = serde_json::to_string(&status)
            .map_err(|e| sqlx::Error::Encode(Box::new(e)))?
            .trim_matches('"').to_string();

        sqlx::query!(
            r#"
            UPDATE publish_jobs
            SET status = $2, result = $3, error = $4, completed_at = CASE WHEN $2 IN ('completed', 'failed') THEN NOW() ELSE completed_at END
            WHERE id = $1
            "#,
            job_id,
            status_str,
            result,
            error
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Save a generated asset
    pub async fn save_asset(&self, asset: &GeneratedAsset) -> Result<(), sqlx::Error> {
        let format_str = serde_json::to_string(&asset.format)
            .map_err(|e| sqlx::Error::Encode(Box::new(e)))?
            .trim_matches('"').to_string();
        let platform_str = asset.platform.as_ref()
            .and_then(|p| serde_json::to_string(p).ok())
            .map(|s| s.trim_matches('"').to_string());
        let hashtags_json = serde_json::to_value(&asset.hashtags)
            .map_err(|e| sqlx::Error::Encode(Box::new(e)))?;

        sqlx::query!(
            r#"
            INSERT INTO generated_assets (id, recipe_id, format, platform, url, caption, hashtags, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            asset.id,
            asset.recipe_id,
            format_str,
            platform_str,
            asset.url,
            asset.caption,
            hashtags_json,
            asset.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get brand theme
    pub async fn get_theme(&self, theme_id: Uuid) -> Result<Option<BrandTheme>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, name, primary_color, secondary_color, font_family, logo_url, template_style, created_at
            FROM brand_themes
            WHERE id = $1
            "#,
            theme_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| BrandTheme {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            primary_color: r.primary_color,
            secondary_color: r.secondary_color,
            font_family: r.font_family,
            logo_url: r.logo_url,
            template_style: serde_json::from_str(&format!("\"{}\"", r.template_style)).unwrap(),
            created_at: r.created_at,
        }))
    }

    /// Get user's themes
    pub async fn get_user_themes(&self, user_id: Uuid) -> Result<Vec<BrandTheme>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, name, primary_color, secondary_color, font_family, logo_url, template_style, created_at
            FROM brand_themes
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| BrandTheme {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            primary_color: r.primary_color,
            secondary_color: r.secondary_color,
            font_family: r.font_family,
            logo_url: r.logo_url,
            template_style: serde_json::from_str(&format!("\"{}\"", r.template_style)).unwrap(),
            created_at: r.created_at,
        }).collect())
    }
}
