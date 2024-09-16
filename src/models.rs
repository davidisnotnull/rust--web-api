use serde::{Deserialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
struct RegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}