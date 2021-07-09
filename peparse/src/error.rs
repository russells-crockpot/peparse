use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Invalid value {} for constant type {}.", value_given, constant_type))]
    InvalidConstant {
        value_given: u64,
        constant_type: String,
    },
    #[snafu(display("Invalid header magic value {}; expected {}", received, expected))]
    InvalidHeaderMagic { expected: String, received: String },
    #[snafu(display("{}", error))]
    SegSourceError { error: segsource::Error },
    #[snafu(display("{}", error))]
    OtherError {
        error: Box<dyn std::error::Error + Sync + Send>,
    },
    #[snafu(display("{}", message))]
    Other { message: String },
}

macro_rules! impl_from {
    ($type:path, $error:ident, $base_error:ident) => {
        impl From<$type> for $base_error {
            fn from(error: $type) -> Self {
                Self::$error { error }
            }
        }
    };
    ($type:path, $error:ident) => {
        impl_from! { $type, $error, Error }
    };
    ($name:ident) => {
        impl_from! { $name, $name }
    };
}

impl_from! {segsource::Error, SegSourceError}

pub type Result<V> = std::result::Result<V, Error>;
