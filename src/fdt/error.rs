use integral_enum::IntegralEnum;
use thiserror::Error;

#[derive(Error, IntegralEnum)]
pub enum SerializeError {
    #[error("Insufficient space for serialization")]
    InsufficientSpace,
}
