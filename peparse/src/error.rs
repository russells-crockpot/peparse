use segsource::Error as SegSourceError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid value {0} for constant type {1}.")]
    InvalidConstant(u64, String),
    #[error("{0}")]
    SegSourceError(SegSourceError),
    #[error("{0}")]
    OtherError(Box<dyn std::error::Error + Sync + Send>),
    #[error("{0}")]
    Other(String),
}

macro_rules! impl_from {
    ($type:ident, $error:ident, $base_error:ident) => {
        impl From<$type> for $base_error {
            fn from(error: $type) -> Self {
                Self::$error(error)
            }
        }
    };
    ($type:ident, $error:ident) => {
        impl_from! { $type, $error, Error }
    };
    ($name:ident) => {
        impl_from! { $name, $name }
    };
}

impl_from! {SegSourceError}

pub type Result<V> = std::result::Result<V, Error>;
