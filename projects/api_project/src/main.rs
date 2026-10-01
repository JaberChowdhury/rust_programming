use axum::{
    Router,
    body::Body,
    http::{Request, Response},
    routing::get,
};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::Span;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact() // Compact formatting keeps it to a single line
        .init();

    let app = Router::new().route("/", get(root)).layer(
        TraceLayer::new_for_http()
            // Disable the default span generation and request-start logs
            .make_span_with(|_req: &Request<Body>| Span::none())
            .on_request(())
            // Single-line summary fired once the response finishes
            .on_response(|res: &Response<Body>, latency: Duration, _span: &Span| {
                tracing::info!(
                    status = res.status().as_u16(),
                    latency = ?latency,
                    "request completed"
                );
            }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
