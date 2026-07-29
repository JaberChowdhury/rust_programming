use axum::{extract::State, routing::post, Json, Router};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    db,
    error::ApiError,
    models::user::{CreateUserRequest, UserResponse},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
    user: UserResponse,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let password_hash = hash(payload.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| ApiError::Internal)?;
        
    let user = db::users::create_user(&state.db, &payload.username, &password_hash).await?;
    
    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let user = db::users::get_user_by_username(&state.db, &payload.username).await?;
    let user = user.ok_or_else(|| ApiError::Auth("Invalid credentials".into()))?;
    
    let valid = verify(payload.password.as_bytes(), &user.password_hash)
        .unwrap_or(false);
        
    if !valid {
        return Err(ApiError::Auth("Invalid credentials".into()));
    }
    
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 24 * 3600; // 24h
        
    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.auth.jwt_secret.as_bytes()),
    ).map_err(|_| ApiError::Internal)?;
    
    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        },
    }))
}
