use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use models::UpdatePreferencesRequest;

use crate::auth::extract_user_id;
use crate::state::AppState;

pub async fn get_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    let profile = state
        .db
        .users()
        .get_profile(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get profile: {}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Profile not found".to_string()))?;

    Ok(Json(profile))
}

pub async fn update_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpdatePreferencesRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    let profile = state
        .db
        .users()
        .update_preferences(user_id, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update profile: {}", e)))?;

    Ok(Json(profile))
}
