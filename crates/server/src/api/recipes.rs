use axum::{
    extract::{State, Path},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use models::{Recipe, ApplyPatchRequest};
use uuid::Uuid;

use crate::auth::extract_user_id;
use crate::state::AppState;

pub async fn list_recipes(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    let recipes = state
        .db
        .recipes()
        .list_by_user(user_id, 50, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to list recipes: {}", e)))?;

    Ok(Json(recipes))
}

pub async fn get_recipe(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    let recipe = state
        .db
        .recipes()
        .get(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, format!("Recipe not found: {}", e)))?;

    Ok(Json(recipe))
}

pub async fn create_recipe(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut recipe): Json<Recipe>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    recipe.user_id = Some(user_id);
    recipe.id = Uuid::new_v4();
    recipe.version = 1;

    let created = state
        .db
        .recipes()
        .create(&recipe)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to create recipe: {}", e)))?;

    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn update_recipe(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(mut req): Json<ApplyPatchRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    req.recipe_id = id;

    let updated = state
        .db
        .recipes()
        .apply_patch(req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to update recipe: {}", e)))?;

    Ok(Json(updated))
}

pub async fn delete_recipe(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _user_id = extract_user_id(&state, headers.get("authorization").and_then(|v| v.to_str().ok()))
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;

    state
        .db
        .recipes()
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete recipe: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
