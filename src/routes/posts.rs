use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};



pub fn posts_route() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(|| async { "Hello, post"}))
}