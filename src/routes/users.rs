use axum::{routing::{get, post}, Router};

use crate::{handlers::{create_user_handler, get_user_handler}, AppState};




pub fn users_route() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user_handler))
        .route("/{id}", get(get_user_handler))
}