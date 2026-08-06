use crate::state::AppState;
use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check(State(state): State<AppState>) -> Json<Value> {
    let db_status = sqlx::query("SELECT 1").execute(&state.db).await.is_ok();

    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "database": if db_status { "ok" } else { "error" }
    }))
}
