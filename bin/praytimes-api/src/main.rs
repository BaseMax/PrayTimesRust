use axum::{routing::post, Router};
use praytimes_api::calculate_handler;

use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/calculate", post(calculate_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
