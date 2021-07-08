#![allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate peparse_internal_macros;

//mod to_expand;
//mod expanded;

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
pub type Va = u32;
