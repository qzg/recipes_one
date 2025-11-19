use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use models::{Claims, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

/// Extract user ID from JWT in request headers
pub async fn extract_user_id(
    state: &AppState,
    auth_header: Option<&str>,
) -> Result<Uuid, AuthError> {
    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AuthError::MissingToken)?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    Uuid::parse_str(&claims.claims.sub)
        .map_err(|_| AuthError::InvalidToken)
}

/// Generate JWT token for user
pub fn generate_token(user_id: Uuid, secret: &str) -> Result<String, AuthError> {
    let claims = Claims::new(user_id, 24); // 24 hour expiration

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenGenerationFailed)
}

/// Hash password with bcrypt
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| AuthError::HashingFailed)
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    bcrypt::verify(password, hash)
        .map_err(|_| AuthError::VerificationFailed)
}

#[derive(Debug, Serialize)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    TokenGenerationFailed,
    HashingFailed,
    VerificationFailed,
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::TokenGenerationFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token"),
            AuthError::HashingFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password"),
            AuthError::VerificationFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to verify password"),
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
