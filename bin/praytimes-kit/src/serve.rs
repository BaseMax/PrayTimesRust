use axum::routing::post;
use axum::{Json, Router};
use std::net::SocketAddr;
use tracing::Level;

use praytimes::types::FormattedTimes;
use tower_http::trace;

use tower_http::trace::TraceLayer;

use crate::base::{calculate, CalculationInputs};

pub async fn calculate_handler(Json(payload): Json<CalculationInputs>) -> Json<FormattedTimes> {
    calculate(payload).into()
}

pub async fn serve() {
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
