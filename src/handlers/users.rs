use crate::{error::{AppResult, AppError}, state::AppState};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use keycloak::types::UserRepresentation;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserQuery {
    search: Option<String>,
    username: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    enabled: Option<bool>,
    email_verified: Option<bool>,
    #[serde(rename = "firstResult")]
    first_result: Option<i32>,
    #[serde(rename = "maxResults")]
    max_results: Option<i32>,
}

pub async fn list_users(
    State(state): State<AppState>,
    Path(realm): Path<String>,
    Query(query): Query<UserQuery>,
) -> AppResult<Json<Vec<UserRepresentation>>> {
    let admin = state.keycloak_admin.read().await;
    let users = admin
        .realm_users_get(
            &realm,
            None, // briefRepresentation
            query.email.as_deref(),
            query.email_verified,
            query.enabled,
            None, // exact
            query.first_name.as_deref(),
            query.first_result,
            None, // idpAlias
            None, // idpUserId
            query.last_name.as_deref(),
            query.max_results,
            None, // q
            query.search.as_deref(),
            query.username.as_deref(),
        )
        .await?;
    Ok(Json(users))
}

pub async fn create_user(
    State(state): State<AppState>,
    Path(realm): Path<String>,
    Json(user): Json<UserRepresentation>,
) -> AppResult<Json<serde_json::Value>> {
    let admin = state.keycloak_admin.read().await;
    let response = admin.realm_users_post(&realm, user).await?;
    
    let user_id = response
        .to_id()
        .ok_or_else(|| AppError::InternalError("Failed to get user ID from response".into()))?;
    
    Ok(Json(serde_json::json!({
        "id": user_id,
        "message": "User created successfully"
    })))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path((realm, user_id)): Path<(String, String)>,
) -> AppResult<Json<UserRepresentation>> {
    let admin = state.keycloak_admin.read().await;
    let user = admin.realm_users_with_user_id_get(&realm, &user_id).await?;
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path((realm, user_id)): Path<(String, String)>,
    Json(user): Json<UserRepresentation>,
) -> AppResult<Json<serde_json::Value>> {
    let admin = state.keycloak_admin.read().await;
    admin
        .realm_users_with_user_id_put(&realm, &user_id, user)
        .await?;
    Ok(Json(serde_json::json!({
        "message": "User updated successfully"
    })))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path((realm, user_id)): Path<(String, String)>,
) -> AppResult<Json<serde_json::Value>> {
    let admin = state.keycloak_admin.read().await;
    admin
        .realm_users_with_user_id_delete(&realm, &user_id)
        .await?;
    Ok(Json(serde_json::json!({
        "message": "User deleted successfully"
    })))
}