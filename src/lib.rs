#![doc = include_str!("../README.md")]

/// # Flattened device tree
///
/// Contains
/// - `NodeBuf`: owned version of the `Node`
/// - `Node`: borrowed fdt node
/// - Search filters like `AnyRegion`, needed for the
///   `Node::find` implementation
///
/// Both are marked as `repr(transparent)` to the `*mut
/// fdt_node`, `fdt_node` accordingly.
pub mod fdt;

/// # Virtual machine instance
///
/// Refer to the `Instance` struct for more information.
/// With the virtual machine instance you can:
///
/// - Do read/writes to the RAM
/// - Start/stop/pause VM execution
/// - Load dtb/kernel/bootrom
pub mod instance;

/// # CPU Handle
pub mod cpu_handle;

/// # Device
///
/// Anything that is related to the mmio devices. Refer to
/// the `mmio` module for the `Device` struct or the `type_`
/// for the `DeviceType` struct.
pub mod dev;

/// # Structure builders
///
/// Builder pattern implementation for various structs.
pub mod builders;

/// # Errors
///
/// This module contains error enums for actions that can
/// fail.
pub mod error;

/// # Types
///
/// Sound and type-safe wrappers around handles/callbacks
pub mod types;

/// # Prelude: contains everything for the quick-start
pub mod prelude;

mod declmacro;
mod internal_utils;

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub use paste as __paste;
pub use rvvm_macro as macros;
pub use rvvm_sys as ffi;
