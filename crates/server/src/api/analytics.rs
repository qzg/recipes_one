use axum::{
    extract::{State, Path},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use models::{LogEventRequest, RecordConversionRequest, AnalyticsEvent, Conversion};
use uuid::Uuid;
use chrono::Utc;

use crate::auth::extract_user_id;
use crate::state::AppState;

pub async fn log_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LogEventRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .ok(); // Optional auth for some events

    let event = AnalyticsEvent {
        id: Uuid::new_v4(),
        user_id,
        recipe_id: req.recipe_id,
        event_type: req.event_type,
        platform: req.platform,
        metadata: req.metadata,
        timestamp: Utc::now(),
        session_id: None,
    };

    state
        .db
        .analytics()
        .log_event(&event)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to log event: {}", e)))?;

    Ok(StatusCode::CREATED)
}

pub async fn record_conversion(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RecordConversionRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    let conversion = Conversion {
        id: Uuid::new_v4(),
        user_id,
        recipe_id: req.recipe_id,
        revenue_cents: req.revenue_cents,
        platform: req.platform,
        conversion_type: req.conversion_type,
        metadata: req.metadata,
        timestamp: Utc::now(),
    };

    state
        .db
        .analytics()
        .record_conversion(&conversion)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to record conversion: {}", e)))?;

    Ok(StatusCode::CREATED)
}

pub async fn get_recipe_analytics(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    let analytics = state
        .db
        .analytics()
        .get_recipe_analytics(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get analytics: {}", e)))?;

    Ok(Json(analytics))
}
