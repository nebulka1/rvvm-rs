use integral_enum::IntegralEnum;
use thiserror::Error;

#[derive(Error, IntegralEnum)]
pub enum SerializeError {
    InsufficientSpace,
}
