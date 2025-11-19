use serde::{Deserialize, Serialize};

/// Configurable OpenAI model selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OpenAIModel {
    /// For realtime dialogue and orchestration
    RealtimeMini,
    /// For batch OCR, high-detail parsing, structured outputs
    Gpt4Mini,
    /// For complex reasoning tasks
    Gpt4,
    /// For vision tasks
    Gpt4Vision,
}

impl OpenAIModel {
    pub fn as_str(&self) -> &str {
        match self {
            OpenAIModel::RealtimeMini => "gpt-4o-realtime-preview",
            OpenAIModel::Gpt4Mini => "gpt-4o-mini",
            OpenAIModel::Gpt4 => "gpt-4o",
            OpenAIModel::Gpt4Vision => "gpt-4o",
        }
    }
}

impl Default for OpenAIModel {
    fn default() -> Self {
        OpenAIModel::Gpt4Mini
    }
}

/// Model configuration with customizable parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model: OpenAIModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model: OpenAIModel::default(),
            temperature: Some(0.7),
            max_tokens: Some(4096),
            top_p: None,
        }
    }
}

impl ModelConfig {
    pub fn new(model: OpenAIModel) -> Self {
        Self {
            model,
            ..Default::default()
        }
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }
}
