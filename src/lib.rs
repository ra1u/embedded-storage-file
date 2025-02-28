
#![doc = include_str!("../README.md")]


mod synhronous;
mod asyncronous;
mod backend_vec;
mod backend_mmap;

pub use synhronous::*;
pub use asyncronous::*;
pub use backend_vec::*;
pub use backend_mmap::*;

