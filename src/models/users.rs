

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}