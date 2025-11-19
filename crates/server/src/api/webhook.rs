use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use models::{RealtimeWebhookEvent, ToolCall};
use serde_json::Value;

use crate::state::AppState;
use crate::tools;

/// Handle OpenAI Realtime webhook events
pub async fn realtime_webhook(
    State(state): State<AppState>,
    Json(event): Json<RealtimeWebhookEvent>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::debug!("Received webhook event: {:?}", event.event_type);

    match event.event_type.as_str() {
        "conversation.item.created" => {
            // Tool call from assistant
            if let Some(item) = event.payload.get("item") {
                if let Some(call_id) = item.get("call_id").and_then(|v| v.as_str()) {
                    if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                        if let Some(arguments) = item.get("arguments") {
                            let tool_call = ToolCall {
                                call_id: call_id.to_string(),
                                name: name.to_string(),
                                arguments: arguments.clone(),
                            };

                            // Execute tool
                            let output = tools::execute_tool(&state, &tool_call).await
                                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Tool execution failed: {}", e)))?;

                            return Ok(Json(output));
                        }
                    }
                }
            }
        }
        "session.created" => {
            // Session initialized, send privileged prompts
            if let Some(session_id) = event.payload.get("session").and_then(|s| s.get("id")).and_then(|v| v.as_str()) {
                tools::inject_privileged_prompt(&state, session_id).await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to inject prompt: {}", e)))?;
            }
        }
        _ => {
            tracing::debug!("Unhandled event type: {}", event.event_type);
        }
    }

    Ok(Json(serde_json::json!({ "status": "ok" })))
}
