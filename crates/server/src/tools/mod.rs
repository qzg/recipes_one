mod recipe_tools;
mod cost_tools;
mod remix_tools;
mod publisher_tools;
mod image_tools;
mod prompts;

use models::{ToolCall, ToolOutput};
use crate::state::AppState;

pub use prompts::inject_privileged_prompt;

/// Execute a tool call from OpenAI Realtime
pub async fn execute_tool(
    state: &AppState,
    tool_call: &ToolCall,
) -> Result<ToolOutput, Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("Executing tool: {}", tool_call.name);

    let output = match tool_call.name.as_str() {
        // Recipe tools
        "get_recipe" => recipe_tools::get_recipe(state, &tool_call.arguments).await?,
        "apply_recipe_patch" => recipe_tools::apply_recipe_patch(state, &tool_call.arguments).await?,
        "create_ui_nudges" => recipe_tools::create_ui_nudges(state, &tool_call.arguments).await?,
        "merge_recipe" => recipe_tools::merge_recipe(state, &tool_call.arguments).await?,
        "persist_user_prefs" => recipe_tools::persist_user_prefs(state, &tool_call.arguments).await?,
        "generate_shopping_list" => recipe_tools::generate_shopping_list(state, &tool_call.arguments).await?,

        // Image tools
        "extract_recipe_from_image" => image_tools::extract_recipe_from_image(state, &tool_call.arguments).await?,
        "detect_pantry_items" => image_tools::detect_pantry_items(state, &tool_call.arguments).await?,

        // Cost tools
        "pricing_compute" => cost_tools::pricing_compute(state, &tool_call.arguments).await?,
        "cost_cutter" => cost_tools::cost_cutter(state, &tool_call.arguments).await?,
        "enrich_recipe" => cost_tools::enrich_recipe(state, &tool_call.arguments).await?,

        // Remix tools
        "remix_improvise" => remix_tools::remix_improvise(state, &tool_call.arguments).await?,
        "remix_cultural_twist" => remix_tools::remix_cultural_twist(state, &tool_call.arguments).await?,
        "remix_healthy_mode" => remix_tools::remix_healthy_mode(state, &tool_call.arguments).await?,

        // Publisher tools
        "publish_recipe_card" => publisher_tools::publish_recipe_card(state, &tool_call.arguments).await?,
        "publish_social_media" => publisher_tools::publish_social_media(state, &tool_call.arguments).await?,

        _ => {
            tracing::warn!("Unknown tool: {}", tool_call.name);
            serde_json::json!({
                "error": format!("Unknown tool: {}", tool_call.name)
            })
        }
    };

    Ok(ToolOutput {
        call_id: tool_call.call_id.clone(),
        output,
    })
}
