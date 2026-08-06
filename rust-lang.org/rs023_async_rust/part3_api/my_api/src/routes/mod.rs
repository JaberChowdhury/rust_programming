use axum::Router;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, request_id::MakeRequestUuid, trace::TraceLayer,
};

use crate::state::AppState;

pub mod auth;
pub mod health;
pub mod users;

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .merge(health::router())
        .merge(auth::router())
        .merge(users::router());

    Router::new()
        .nest("/api", api_routes)
        // Add middleware
        .layer(
            tower::ServiceBuilder::new()
                .layer(tower_http::request_id::SetRequestIdLayer::x_request_id(
                    MakeRequestUuid,
                ))
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new())
                .into_inner(),
        )
        .with_state(state)
}
