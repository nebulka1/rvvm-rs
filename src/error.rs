use integral_enum::IntegralEnum;
use thiserror::Error;

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum InstanceCreateError {
    #[error("Failed to allocate memory for the instance")]
    FailedToAllocate,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum InstancePauseError {
    #[error("Instance is not running")]
    NotRunning,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum InstanceStartError {
    #[error("Instance is already running")]
    AlreadyRunning,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum DtbDumpError {
    #[error("Failed to open destination file for write")]
    FailedToOpenFile,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum MemoryAccessError {
    #[error("Requested range is out-of-bounds")]
    OutOfBounds,
}

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum DeviceAttachError {
    #[error("Tried to attach device to already occupied region")]
    RegionIsOccupied,
}
