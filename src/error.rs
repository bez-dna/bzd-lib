use tonic::Status;

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
}

// TODO: надо разобраться с этим поглууубже
#[macro_export]
macro_rules! impl_from_other {
    ($($err:ty),+ $(,)?) => {
        $(
            impl From<$err> for AppError {
                fn from(_: $err) -> Self {
                    Self::Internal
                }
            }
        )+
    };
}

impl_from_other!(
    prost::EncodeError,
    prost::DecodeError,
    serde_json::Error,
    uuid::Error,
);

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation => Self::invalid_argument(error.to_string()),
            AppError::NotFound => Self::not_found(error.to_string()),
            AppError::Forbidden => Self::permission_denied(error.to_string()),
            AppError::Internal => Self::internal(error.to_string()),
        }
    }
}
