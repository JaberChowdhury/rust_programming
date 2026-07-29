use axum::{
    extract::{Path, Query, State},
    routing::{get, put, delete},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db,
    error::ApiError,
    middleware::auth::AuthUser,
    models::user::{UpdateUserRequest, UserResponse},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}

async fn list_users(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let page = pagination.page.unwrap_or(1).max(1);
    let per_page = pagination.per_page.unwrap_or(20).clamp(1, 100);
    
    let users = db::users::list_users(&state.db, page, per_page).await?;
    
    let response = users.into_iter().map(|u| UserResponse {
        id: u.id,
        username: u.username,
        created_at: u.created_at,
        updated_at: u.updated_at,
    }).collect();
    
    Ok(Json(response))
}

async fn get_user(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = db::users::get_user(&state.db, id).await?
        .ok_or(ApiError::NotFound)?;
        
    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

async fn update_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    if auth.user_id != id {
        return Err(ApiError::Auth("Forbidden: Owner only".into()));
    }
    
    let user = db::users::update_user(&state.db, id, &payload.username).await?
        .ok_or(ApiError::NotFound)?;
        
    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

async fn delete_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, ApiError> {
    if auth.user_id != id {
        return Err(ApiError::Auth("Forbidden: Owner only".into()));
    }
    
    db::users::delete_user(&state.db, id).await?;
    
    Ok(Json(()))
}
