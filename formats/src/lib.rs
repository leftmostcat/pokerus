#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

pub mod crypto;

mod pokemon;
pub use pokemon::*;

mod saves;
pub use saves::*;

pub(crate) mod utils;

pub mod gen1;
pub mod gen6;
