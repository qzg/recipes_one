use crate::{OpenAIError, ModelConfig};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Main OpenAI API client
#[derive(Clone)]
pub struct OpenAIClient {
    api_key: String,
    client: Client,
    base_url: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// Create ephemeral token for Realtime API
    pub async fn create_ephemeral_token(&self, model: &str) -> Result<EphemeralTokenResponse, OpenAIError> {
        let request = EphemeralTokenRequest {
            model: model.to_string(),
            voice: "alloy".to_string(),
        };

        let response = self
            .client
            .post(&format!("{}/realtime/sessions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(OpenAIError::ApiError(error_text));
        }

        let token_response: EphemeralTokenResponse = response.json().await?;
        Ok(token_response)
    }

    /// Upload file to OpenAI Files API
    pub async fn upload_file(&self, file_data: Vec<u8>, filename: &str, purpose: &str) -> Result<FileUploadResponse, OpenAIError> {
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(file_data).file_name(filename.to_string()))
            .text("purpose", purpose.to_string());

        let response = self
            .client
            .post(&format!("{}/files", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(OpenAIError::ApiError(error_text));
        }

        let file_response: FileUploadResponse = response.json().await?;
        Ok(file_response)
    }

    /// Call chat completion API
    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        config: ModelConfig,
    ) -> Result<ChatCompletionResponse, OpenAIError> {
        let request = ChatCompletionRequest {
            model: config.model.as_str().to_string(),
            messages,
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            top_p: config.top_p,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(OpenAIError::ApiError(error_text));
        }

        let completion: ChatCompletionResponse = response.json().await?;
        Ok(completion)
    }

    /// Call structured output API with JSON schema
    pub async fn structured_output(
        &self,
        messages: Vec<ChatMessage>,
        schema: Value,
        config: ModelConfig,
    ) -> Result<Value, OpenAIError> {
        let mut request = serde_json::json!({
            "model": config.model.as_str(),
            "messages": messages,
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "name": "recipe_response",
                    "schema": schema,
                    "strict": true
                }
            }
        });

        if let Some(temp) = config.temperature {
            request["temperature"] = serde_json::json!(temp);
        }
        if let Some(tokens) = config.max_tokens {
            request["max_tokens"] = serde_json::json!(tokens);
        }

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(OpenAIError::ApiError(error_text));
        }

        let completion: ChatCompletionResponse = response.json().await?;
        let content = completion.choices.first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| OpenAIError::InvalidResponse("No content in response".to_string()))?;

        let structured: Value = serde_json::from_str(content)?;
        Ok(structured)
    }
}

// Request/Response types
#[derive(Debug, Serialize)]
struct EphemeralTokenRequest {
    model: String,
    voice: String,
}

#[derive(Debug, Deserialize)]
pub struct EphemeralTokenResponse {
    pub client_secret: ClientSecret,
}

#[derive(Debug, Deserialize)]
pub struct ClientSecret {
    pub value: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    pub id: String,
    pub filename: String,
    pub bytes: u64,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatResponseMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponseMessage {
    pub role: String,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
