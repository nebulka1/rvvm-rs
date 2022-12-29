pub mod error;
pub mod instance;

pub mod dest_ptr;
pub mod dev;
pub mod utils;

pub use rvvm_sys as ffi;

#[cfg(test)]
mod tests;
