use axum::Router;
use sqlx::{Pool, Postgres};

use crate::routes::{posts::posts_route, users::users_route};

pub mod users;
pub mod posts;


pub fn api_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .nest("/users", users_route())
        .nest("/posts", posts_route())
}