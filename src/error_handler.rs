use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorHandling {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Validation error")]
    ValidationError(#[from] validator::ValidationErrors),
}