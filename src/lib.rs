pub mod fdt;
pub mod instance;

pub mod builders;
pub mod error;

pub use rvvm_sys as ffi;

#[cfg(test)]
mod tests;

mod declmacro;
