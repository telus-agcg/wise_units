#![allow(clippy::unused_async)]

use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wise_units::Measurement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    lhs: Measurement,
    rhs: Measurement,
}

#[debug_handler]
pub async fn mul(Json(params): Json<Params>) -> impl IntoResponse {
    debug!("Multiplying {} * {}", &params.lhs, &params.rhs);

    format::json(params.lhs * params.rhs)
}

pub fn routes() -> Routes {
    Routes::new().prefix("mul").add("/", post(mul))
}

