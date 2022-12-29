use integral_enum::IntegralEnum;
use thiserror::Error;

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum InstanceCreateError {
    #[error("Failed to allocate machine instance")]
    FailedToAllocate,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum MemError {
    #[error("Invalid memory region specified")]
    InvalidMemoryRegion,

    #[error("Buffer will overflow specified region")]
    TooLongBuffer,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum CCharsCreateFailure {
    #[error("Input has no nul-terminator (\\0)")]
    NoNulTerminator,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum DeviceAttachError {
    #[error(
        "Device address space overlaps another device's address space"
    )]
    DeviceIsOverlapped,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum MmioTypeCreateError {
    #[error("Invalid device name")]
    InvalidName,
}
