pub type Error = anyhow::Error;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("VALIDATION")]
    Validation,
    #[error("NOT_FOUND")]
    NotFound,
    #[error("FORBIDDEN")]
    Forbidden,

    #[error("INTERNAL")]
    Internal,
    #[error("UNREACHABLE")]
    Unreachable,
}
