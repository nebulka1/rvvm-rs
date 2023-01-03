use std::ffi::CStr;

pub trait DeviceType {
    type Device;

    /// Get name of the device type
    fn name(&self) -> &CStr;

    /// Remove handler, called when device is being removed
    /// from the instance
    fn remove(&mut self, dev: &mut Self::Device);

    /// TODO
    fn reset(&mut self, dev: &mut Self::Device);

    /// TODO
    fn update(&mut self, dev: &mut Self::Device);
}
