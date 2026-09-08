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

// TODO: надо разобраться с этим поглууубже
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
