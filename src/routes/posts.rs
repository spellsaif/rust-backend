use axum::{routing::get, Router};

use crate::AppState;



pub fn posts_route() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, post"}))
}