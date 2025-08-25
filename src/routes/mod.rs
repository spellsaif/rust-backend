use axum::Router;

use crate::{routes::{posts::posts_route, users::users_route}, AppState};

pub mod users;
pub mod posts;


pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", users_route())
        .nest("/posts", posts_route())
}