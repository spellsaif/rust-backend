use std::env;

use axum::{ response::IntoResponse, routing::{get}, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenvy::dotenv;

use crate::routes::api_routes;

mod routes;
mod handlers;
mod models;

#[derive(Debug, Clone)]
pub struct AppState {
    db: PgPool
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenv().ok(); //Loading .env variables

    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;


    sqlx::migrate!().run(&pool).await?;

    //Create AppState
    let state = AppState {
        db: pool
    };

   let router = Router::new()
                            .route("/health", get(health_handler))
                            .merge(api_routes())                           
                            .with_state(state);

   let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

   axum::serve(listener, router).await.unwrap();

   Ok(())
}


async fn health_handler() -> impl IntoResponse {
    "Server is running".into_response()
} 
