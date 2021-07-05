#![allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros)]

#[macro_use]
extern crate peparse_internal_macros;

pub mod coff;
pub(crate) mod constants;
mod error;
mod headers;
pub use error::*;
pub mod util;
