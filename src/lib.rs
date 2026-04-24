#![doc = include_str!("../README.md")]

mod asyncronous;
mod backend_mmap;
mod backend_vec;
mod synhronous;

pub use asyncronous::*;
pub use backend_mmap::*;
pub use backend_vec::*;
pub use synhronous::*;
