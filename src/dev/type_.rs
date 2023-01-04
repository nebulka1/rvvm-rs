use std::ffi::CStr;

/// Unsafe functions of the DeviceType
///
/// # Safety
///
/// This trait is meant to be unsafe due to internal
/// implementation
pub unsafe trait UnsafeDeviceType {
    fn name(&self) -> &CStr;
}

pub trait DeviceType: UnsafeDeviceType {
    type Device;

    /// TODO
    fn reset(&mut self, dev: &mut Self::Device);

    /// TODO
    fn update(&mut self, dev: &mut Self::Device);
}
