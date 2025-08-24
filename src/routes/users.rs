use axum::{routing::{get, post}, Router};
use sqlx::{Pool, Postgres};

use crate::handlers::{create_user_handler, get_user_handler};




pub fn users_route() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", post(create_user_handler))
        .route("/{id}", get(get_user_handler))
}