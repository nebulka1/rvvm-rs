#![doc = include_str!("../README.md")]

pub mod fdt;
pub mod instance;

pub mod dev;

pub mod builders;
pub mod error;

pub mod types;

pub mod prelude;

mod declmacro;
mod internal_utils;

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub use paste as __paste;
pub use rvvm_macro as macros;
pub use rvvm_sys as ffi;
