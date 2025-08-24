use axum::Router;
use sqlx::{Pool, Postgres};

use crate::routes::users::users_route;

pub mod users;


pub fn api_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .nest("/users", users_route())
}