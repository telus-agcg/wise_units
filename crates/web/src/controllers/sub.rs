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
pub async fn sub(Json(params): Json<Params>) -> Result<impl IntoResponse> {
    debug!("Subtracting {} - {}", &params.lhs, &params.rhs);

    match params.lhs - params.rhs {
        Ok(result) => format::json(result),
        Err(e) => format::json(e),
    }
}

pub fn routes() -> Routes {
    Routes::new().prefix("sub").add("/", post(sub))
}
