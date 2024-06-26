mod convert;

use axum::{routing::post, Router};
use convert::convert;

pub fn create_v1_routes() -> Router {
    Router::new().route("/convert", post(convert))
}
