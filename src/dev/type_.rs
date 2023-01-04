use std::ffi::CStr;

pub trait DeviceType {
    type Device;

    /// Get name of the device type
    fn name(&self) -> &CStr;

    /// Remove handler, called when device is being removed
    /// from the instance
    fn remove(_dev: &mut Self::Device) {}

    /// TODO
    fn reset(_dev: &mut Self::Device) {}

    /// TODO
    fn update(_dev: &mut Self::Device) {}
}

pub unsafe trait DeviceTypeExt: DeviceType {
    fn new() -> Self;
}
