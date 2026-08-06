use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(err) => {
                if let sqlx::Error::Database(db_err) = &err {
                    if db_err.is_unique_violation() {
                        (StatusCode::CONFLICT, "Resource already exists".to_string())
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Database error".to_string(),
                        )
                    }
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    )
                }
            }
            ApiError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
            ),
            ApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
