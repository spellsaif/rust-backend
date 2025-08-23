use axum::{extract::Path, response::IntoResponse, routing::{get, post}, Json, Router};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    name: String,
    email: String,
}



#[tokio::main]
async fn main() {

   let router = Router::new()
                            .route("/health", get(health_handler))
                            .route("/users", post(create_user_handler))
                            .route("/users/{id}", get(get_user_handler));

   let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

   axum::serve(listener, router).await.unwrap();
}


async fn health_handler() -> impl IntoResponse {
    "Server is running".into_response()
} 

async fn create_user_handler(Json(payload): Json<User>) -> impl IntoResponse {
    let user = User {name: payload.name, email: payload.email};

    Json(user).into_response()
}

async fn get_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    format!("User id: {}", id).into_response()
}