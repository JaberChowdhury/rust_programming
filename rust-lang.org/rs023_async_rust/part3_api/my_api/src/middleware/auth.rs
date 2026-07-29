use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::{error::ApiError, routes::auth::Claims, state::AppState};

pub struct AuthUser {
    pub user_id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| ApiError::Auth("Missing or invalid token".into()))?;

        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret(state.config.auth.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Auth("Invalid token".into()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| ApiError::Auth("Invalid user ID in token".into()))?;

        Ok(AuthUser { user_id })
    }
}
