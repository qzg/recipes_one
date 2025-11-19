use crate::state::AppState;
use models::{PublishJobType, JobStatus, GeneratedAsset, CardFormat, SocialPlatform};
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;

pub async fn publish_recipe_card(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;
    let recipe_version: i32 = serde_json::from_value(args["recipe_version"].clone())?;
    let format: CardFormat = serde_json::from_value(args["format"].clone())?;
    let theme_id: Option<Uuid> = serde_json::from_value(args.get("theme_id").cloned().unwrap_or(Value::Null))?;

    let recipe = state.db.recipes().get(recipe_id).await?;
    let user_id = recipe.user_id.ok_or("Recipe has no owner")?;

    // Create publish job
    let job = state.db.publisher().create_job(
        user_id,
        recipe_id,
        recipe_version,
        PublishJobType::RecipeCard,
    ).await?;

    // In a real implementation, this would:
    // 1. Load brand theme if specified
    // 2. Generate recipe card using template engine or AI
    // 3. Upload to CDN/S3
    // 4. Save generated asset
    // 5. Update job status

    // For now, simulate success
    let asset_url = format!("https://cdn.example.com/recipes/{}/card.png", recipe_id);

    let asset = GeneratedAsset {
        id: Uuid::new_v4(),
        recipe_id,
        format: format.clone(),
        platform: None,
        url: asset_url.clone(),
        caption: Some(recipe.title.clone()),
        hashtags: vec!["recipe".to_string(), "cooking".to_string()],
        created_at: Utc::now(),
    };

    state.db.publisher().save_asset(&asset).await?;

    state.db.publisher().update_job_status(
        job.id,
        JobStatus::Completed,
        Some(serde_json::to_value(&asset)?),
        None,
    ).await?;

    Ok(serde_json::to_value(asset)?)
}

pub async fn publish_social_media(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let recipe_id: Uuid = serde_json::from_value(args["recipe_id"].clone())?;
    let recipe_version: i32 = serde_json::from_value(args["recipe_version"].clone())?;
    let platforms: Vec<SocialPlatform> = serde_json::from_value(args["platforms"].clone())?;
    let theme_id: Option<Uuid> = serde_json::from_value(args.get("theme_id").cloned().unwrap_or(Value::Null))?;

    let recipe = state.db.recipes().get(recipe_id).await?;
    let user_id = recipe.user_id.ok_or("Recipe has no owner")?;

    // Create publish job
    let job = state.db.publisher().create_job(
        user_id,
        recipe_id,
        recipe_version,
        PublishJobType::SocialPost,
    ).await?;

    let mut assets = vec![];

    for platform in platforms {
        // In a real implementation:
        // 1. Generate platform-specific caption (AI)
        // 2. Create optimized image for platform dimensions
        // 3. Generate hashtags based on recipe content
        // 4. Upload assets to CDN

        let caption = generate_social_caption(&recipe, &platform);
        let hashtags = generate_hashtags(&recipe);

        let asset_url = format!(
            "https://cdn.example.com/recipes/{}/{:?}.jpg",
            recipe_id,
            platform
        );

        let asset = GeneratedAsset {
            id: Uuid::new_v4(),
            recipe_id,
            format: match platform {
                SocialPlatform::Instagram => CardFormat::SocialSquare,
                SocialPlatform::TikTok => CardFormat::SocialStory,
                _ => CardFormat::SocialLandscape,
            },
            platform: Some(platform),
            url: asset_url,
            caption: Some(caption),
            hashtags: hashtags.clone(),
            created_at: Utc::now(),
        };

        state.db.publisher().save_asset(&asset).await?;
        assets.push(asset);
    }

    state.db.publisher().update_job_status(
        job.id,
        JobStatus::Completed,
        Some(serde_json::to_value(&assets)?),
        None,
    ).await?;

    Ok(serde_json::to_value(assets)?)
}

fn generate_social_caption(recipe: &models::Recipe, platform: &SocialPlatform) -> String {
    match platform {
        SocialPlatform::Instagram => format!(
            "🍽️ {}\n\n{}\n\nServes {} | Easy to make",
            recipe.title,
            recipe.description.as_deref().unwrap_or("Delicious recipe!"),
            recipe.servings
        ),
        SocialPlatform::TikTok => format!(
            "Quick {} recipe! Serves {} 🔥",
            recipe.title,
            recipe.servings
        ),
        _ => format!("{} - Serves {}", recipe.title, recipe.servings),
    }
}

fn generate_hashtags(recipe: &models::Recipe) -> Vec<String> {
    let mut tags = vec![
        "recipe".to_string(),
        "cooking".to_string(),
        "foodie".to_string(),
        "homemade".to_string(),
    ];

    // Add recipe-specific tags based on title/ingredients
    let title_lower = recipe.title.to_lowercase();
    if title_lower.contains("chicken") {
        tags.push("chicken".to_string());
    }
    if title_lower.contains("pasta") {
        tags.push("pasta".to_string());
    }
    if title_lower.contains("healthy") {
        tags.push("healthyfood".to_string());
    }

    tags
}
