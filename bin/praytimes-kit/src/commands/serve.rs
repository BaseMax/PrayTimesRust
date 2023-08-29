use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tracing::Level;

use chrono::{FixedOffset, Local, NaiveDate, Utc};
use praytimes::{
    types::{FormattedTimes, Location, TuneOffsets},
    Calculator,
};

use tower_http::trace;

use tower_http::trace::TraceLayer;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::base::{CustomizableParams, Zone};

async fn calculate_handler(
    Json(payload): Json<CalculationInputs>,
) -> Result<Json<FormattedTimes>, (StatusCode, Json<Value>)> {
    let result = Calculator::new(
        payload.parameters.get_params(),
        payload.tuning.unwrap_or_default(),
    )
    .calculate(&payload.location, &payload.date);

    let formatted = match payload.zone {
        Zone::Local => result.format_times(&payload.format, &Local),
        Zone::Utc => result.format_times(&payload.format, &Utc),
        Zone::Fixed(o) => result.format_times(
            &payload.format,
            &FixedOffset::east_opt(o).ok_or((
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"invalid time zone"})),
            ))?,
        ),
    };

    Ok(Json(formatted))
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

    let host: Ipv4Addr = std::env::var("HOST")
        .unwrap_or("127.0.0.1".into())
        .parse()
        .expect("Invalid host");
    let port: u16 = std::env::var("PORT")
        .unwrap_or("3535".into())
        .parse()
        .expect("Invalid port");
    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(host, port));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn default_timezone() -> Zone {
    Zone::Local
}

fn default_format() -> String {
    "%+".into()
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalculationInputs {
    #[serde(default = "default_format")]
    pub format: String,
    pub date: NaiveDate,
    pub location: Location,
    pub parameters: CustomizableParams,
    pub tuning: Option<TuneOffsets>,
    #[serde(default = "default_timezone")]
    pub zone: Zone,
}
