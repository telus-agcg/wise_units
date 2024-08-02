#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wise_units::{Convertible, Measurement, Unit};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    lhs: Measurement,
    rhs: Unit,
}

#[debug_handler]
pub async fn convert(Json(params): Json<Params>) -> Result<impl IntoResponse> {
    debug!("Converting {} to {}", &params.lhs, &params.rhs);

    match params.lhs.convert_to(&params.rhs) {
        Ok(result) => format::json(result),
        Err(e) => format::json(e),
    }
}

pub fn routes() -> Routes {
    Routes::new().prefix("convert").add("/", post(convert))
}
