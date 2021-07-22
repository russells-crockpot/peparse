use crate::Rva;
use snafu::Snafu;
use std::{io, string::FromUtf8Error};

macro_rules! error_item {
    ($name:ident {$($item:item),*}) => {
        $name {
            $($item:item,)*
            #[cfg(feature = "backtrace")]
            backtrace: snafu::Backtrace,
        }
    }
}

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Invalid value {} for constant type {}.", value_given, constant_type))]
    InvalidConstant {
        value_given: u64,
        constant_type: String,
    },
    #[snafu(display("Invalid header magic value {}; expected {}", received, expected))]
    InvalidHeaderMagic { expected: String, received: String },
    #[snafu(display("Invalid RVA: {:08x}", rva))]
    InvalidRva { rva: Rva },
    #[snafu(display("{}", error))]
    SegSourceError { error: segsource::Error },
    #[snafu(display("{}", message))]
    EncodingError { message: String },
    #[snafu(display("{}", error))]
    IoError { error: io::Error },
    #[snafu(display("{}", error))]
    OtherError {
        error: Box<dyn std::error::Error + Sync + Send>,
    },
    #[snafu(display("{}", message))]
    Other { message: String },
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::EncodingError {
            message: format!(
                "Received invalid UTF-8 data: {}",
                hex::encode(error.as_bytes())
            ),
        }
    }
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
impl_from! {io::Error, IoError}

pub type Result<V> = std::result::Result<V, Error>;
