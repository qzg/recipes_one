use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecipeError {
    #[error("Recipe not found: {0}")]
    NotFound(String),

    #[error("Version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: i32, actual: i32 },

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Recipe is invalid: {0}")]
    InvalidRecipe(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    NotFound(String),

    #[error("Session expired")]
    Expired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Session limit reached")]
    ConcurrencyLimit,
}

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Recipe(#[from] RecipeError),

    #[error(transparent)]
    Session(#[from] SessionError),

    #[error(transparent)]
    OpenAI(#[from] OpenAIError),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

// Implement common conversions
impl From<serde_json::Error> for RecipeError {
    fn from(err: serde_json::Error) -> Self {
        RecipeError::SerializationError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Recipe(RecipeError::SerializationError(err.to_string()))
    }
}
