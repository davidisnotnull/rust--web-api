use serde::{Serialize, Deserialize};
use uuid::Uuid;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    #[validate(email)]
    pub email: String,
}