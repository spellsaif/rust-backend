use axum::{extract::{Path, State}, response::IntoResponse, Json};
use sqlx::PgPool;



pub async fn create_user_handler(State(pool): State<PgPool> ,Json(payload): Json<crate::models::User>) -> impl IntoResponse {

    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&payload.name)
        .bind(&payload.email)
        .execute(&pool)
        .await
        .unwrap();
    Json(payload).into_response()
}

pub async fn get_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    format!("User id: {}", id).into_response()
}