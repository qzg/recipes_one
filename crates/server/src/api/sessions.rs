use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use models::{CreateSessionRequest, CreateSessionResponse};
use chrono::{Utc, Duration};
use openai::{RealtimeClient, OpenAIModel};

use crate::auth::extract_user_id;
use crate::state::AppState;

pub async fn create_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateSessionRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Extract and verify user
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    // Create session in database
    let expires_at = Utc::now() + Duration::hours(1);
    let session = state
        .db
        .sessions()
        .create(user_id, req.recipe_id, expires_at)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create session: {}", e)))?;

    // Create ephemeral token from OpenAI
    let realtime_client = RealtimeClient::new(state.openai.clone());
    let (ephemeral_token, _expires_at) = realtime_client
        .create_session(OpenAIModel::RealtimeMini.as_str())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create ephemeral token: {}", e)))?;

    // Update session with OpenAI details
    // Note: We don't have openai_session_id yet, it comes from the client after connection
    state
        .db
        .sessions()
        .update_openai_session(&session.id.to_string(), "pending", &ephemeral_token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update session: {}", e)))?;

    Ok(Json(CreateSessionResponse {
        session_id: session.id,
        ephemeral_token,
        expires_at: session.expires_at,
    }))
}
