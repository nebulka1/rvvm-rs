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
pub enum DeviceAttachError {
    #[error("Tried to attach device to already occupied region")]
    RegionIsOccupied,

    #[error("Can't attach device to the running machine")]
    VmIsRunning,
}
