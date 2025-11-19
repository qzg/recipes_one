use crate::state::AppState;
use openai::{VisionClient, OpenAIModel};
use openai::vision::ImageDetail;
use serde_json::Value;

pub async fn extract_recipe_from_image(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let image_url = args.get("image_url")
        .and_then(|v| v.as_str())
        .ok_or("Missing image_url")?;

    let detail = args.get("detail")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
            "low" => Some(ImageDetail::Low),
            "high" => Some(ImageDetail::High),
            "auto" => Some(ImageDetail::Auto),
            _ => None,
        })
        .unwrap_or(ImageDetail::Low);

    let model = args.get("model")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
            "gpt4_vision" => Some(OpenAIModel::Gpt4Vision),
            "gpt4" => Some(OpenAIModel::Gpt4),
            _ => None,
        })
        .unwrap_or(OpenAIModel::Gpt4Vision);

    let vision_client = VisionClient::new(state.openai.clone());
    let recipe = vision_client.extract_recipe_from_image(image_url, detail, model).await
        .map_err(|e| format!("Failed to extract recipe: {}", e))?;

    Ok(serde_json::to_value(recipe)?)
}

pub async fn detect_pantry_items(
    state: &AppState,
    args: &Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let image_url = args.get("image_url")
        .and_then(|v| v.as_str())
        .ok_or("Missing image_url")?;

    let detail = args.get("detail")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
            "low" => Some(ImageDetail::Low),
            "high" => Some(ImageDetail::High),
            "auto" => Some(ImageDetail::Auto),
            _ => None,
        })
        .unwrap_or(ImageDetail::Low);

    let vision_client = VisionClient::new(state.openai.clone());
    let items = vision_client.detect_pantry_items(image_url, detail, OpenAIModel::Gpt4Vision).await
        .map_err(|e| format!("Failed to detect items: {}", e))?;

    Ok(serde_json::to_value(items)?)
}
