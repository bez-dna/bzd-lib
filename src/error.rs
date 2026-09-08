pub type Error = anyhow::Error;

#[macro_export]
macro_rules! internal_from {
    ($target:ty; $($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for $target {
                fn from(_: $error) -> Self {
                    Self::Internal
                }
            }
        )+
    };
}
