#![allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate peparse_internal_macros;

mod base;
pub use base::*;

pub mod archive;
pub mod coff;
mod error;
pub mod image;
pub use error::*;
pub mod sections;
pub mod util;

pub type Rva = u32;

/// A virtual address. Unlike with [`Rva`]s, [`Va`]s have a different size depending on if we're
/// dealing with a PE32 or a PE32+. We use the larger one since a u32 can be represented by a u64.
pub type Va = u64;
