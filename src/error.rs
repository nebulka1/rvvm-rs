use integral_enum::IntegralEnum;
use thiserror::Error;

#[derive(IntegralEnum, Error)]
#[enum_disable(display)]
pub enum InstanceCreateError {
    #[error("Failed to allocate machine instance")]
    FailedToAllocate,
}

#[derive(IntegralEnum, Error)]
pub enum CCharsCreateFailure {
    NoNulTerminator,
}

#[derive(IntegralEnum, Error)]
pub enum MmioTypeCreateError {
    InvalidName,
}
