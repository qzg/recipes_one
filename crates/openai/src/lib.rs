pub mod client;
pub mod realtime;
pub mod vision;
pub mod models_config;

pub use client::OpenAIClient;
pub use realtime::RealtimeClient;
pub use vision::VisionClient;
pub use models_config::{ModelConfig, OpenAIModel};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<reqwest::Error> for OpenAIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAIError::RequestFailed(err.to_string())
    }
}

impl From<serde_json::Error> for OpenAIError {
    fn from(err: serde_json::Error) -> Self {
        OpenAIError::SerializationError(err.to_string())
    }
}
