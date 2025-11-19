use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Recipe card/asset generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRecipeCardRequest {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub format: CardFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CardFormat {
    Png,
    Pdf,
    Html,
    SocialSquare,    // 1:1 for Instagram
    SocialStory,     // 9:16 for Stories
    SocialLandscape, // 16:9 for YouTube/FB
}

/// Social media post generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishSocialMediaRequest {
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub platforms: Vec<SocialPlatform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SocialPlatform {
    Instagram,
    TikTok,
    Pinterest,
    Twitter,
    Facebook,
    YouTube,
}

/// Generated asset (card or social post)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedAsset {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub format: CardFormat,
    pub platform: Option<SocialPlatform>,
    pub url: String, // S3 or CDN URL
    pub caption: Option<String>,
    pub hashtags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Brand theme for visual customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTheme {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub primary_color: String,   // Hex color
    pub secondary_color: String, // Hex color
    pub font_family: String,
    pub logo_url: Option<String>,
    pub template_style: TemplateStyle,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TemplateStyle {
    Minimal,
    Bold,
    Elegant,
    Rustic,
    Modern,
    Playful,
}

/// Publish job record (tracking generation history)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishJob {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recipe_id: Uuid,
    pub recipe_version: i32,
    pub job_type: PublishJobType,
    pub status: JobStatus,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PublishJobType {
    RecipeCard,
    SocialPost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_request() {
        let request = PublishRecipeCardRequest {
            recipe_id: Uuid::new_v4(),
            recipe_version: 1,
            format: CardFormat::Png,
            theme_id: None,
        };

        assert_eq!(request.format, CardFormat::Png);
    }
}
