use crate::{OpenAIClient, OpenAIError};
use models::{ToolCall, ToolOutput};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Realtime API client for session control
#[derive(Clone)]
pub struct RealtimeClient {
    client: OpenAIClient,
}

impl RealtimeClient {
    pub fn new(client: OpenAIClient) -> Self {
        Self { client }
    }

    /// Create an ephemeral token for client WebRTC connection
    pub async fn create_session(&self, model: &str) -> Result<(String, i64), OpenAIError> {
        let response = self.client.create_ephemeral_token(model).await?;
        Ok((response.client_secret.value, response.client_secret.expires_at))
    }

    /// Send session update (privileged prompts, server-side only)
    pub async fn update_session(
        &self,
        session_id: &str,
        instructions: &str,
        tools: Vec<ToolDefinition>,
    ) -> Result<(), OpenAIError> {
        // This would be sent via webhook or server events API
        // Implementation depends on OpenAI's webhook system
        tracing::info!("Updating session {} with {} tools", session_id, tools.len());
        Ok(())
    }

    /// Execute a tool call and return output
    pub async fn handle_tool_call(&self, tool_call: &ToolCall) -> Result<ToolOutput, OpenAIError> {
        // This is a placeholder - actual implementation will be in the server
        // when handling webhook events
        Ok(ToolOutput {
            call_id: tool_call.call_id.clone(),
            output: serde_json::json!({
                "success": true,
                "message": "Tool executed"
            }),
        })
    }

    /// Create a response.create event for structured output
    pub fn create_structured_response_event(&self, schema: Value, instructions: &str) -> Value {
        serde_json::json!({
            "type": "response.create",
            "response": {
                "instructions": instructions,
                "modalities": ["text"],
                "output_text_format": {
                    "type": "json_schema",
                    "json_schema": {
                        "name": "recipe_output",
                        "schema": schema,
                        "strict": true
                    }
                }
            }
        })
    }
}

/// Tool definition for Realtime API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

impl ToolDefinition {
    pub fn new(name: &str, description: &str, parameters: Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        }
    }
}

/// Session update request
#[derive(Debug, Serialize)]
pub struct SessionUpdate {
    #[serde(rename = "type")]
    pub event_type: String,
    pub session: SessionConfig,
}

#[derive(Debug, Serialize)]
pub struct SessionConfig {
    pub instructions: String,
    pub tools: Vec<ToolDefinition>,
    pub tool_choice: String,
}

impl SessionUpdate {
    pub fn new(instructions: String, tools: Vec<ToolDefinition>) -> Self {
        Self {
            event_type: "session.update".to_string(),
            session: SessionConfig {
                instructions,
                tools,
                tool_choice: "auto".to_string(),
            },
        }
    }
}
