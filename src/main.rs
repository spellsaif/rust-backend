use axum::{routing::{get, post}, Json, Router};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    name: String,
    email: String,
}



#[tokio::main]
async fn main() {

   let router = Router::new()
                            .route("/health", get(health_handler))
                            .route("/users", post(create_user_handler));

   let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

   axum::serve(listener, router).await.unwrap();
}


async fn health_handler() -> &'static str {
    "Server is running"
} 

async fn create_user_handler(Json(payload): Json<User>) -> Json<User> {
    let user = User {name: payload.name, email: payload.email};

    Json(user)
}