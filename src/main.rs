use std::env;

use axum::{extract::{Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenvy::dotenv;
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    name: String,
    email: String,
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


   let router = Router::new()
                            .route("/health", get(health_handler))
                            .route("/users", post(create_user_handler))
                            .route("/users/{id}", get(get_user_handler))
                            .with_state(pool);

   let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

   axum::serve(listener, router).await.unwrap();

   Ok(())
}


async fn health_handler() -> impl IntoResponse {
    "Server is running".into_response()
} 

async fn create_user_handler(State(pool): State<PgPool> ,Json(payload): Json<User>) -> impl IntoResponse {

    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.email)
        .execute(&pool)
        .await
        .unwrap();
    Json(payload).into_response()
}

async fn get_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    format!("User id: {}", id).into_response()
}