use axum::{routing::get, Router};

pub fn router_init() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
