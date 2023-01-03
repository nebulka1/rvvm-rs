/// # Safety
///
/// This trait is unsafe due to untyped raw pointer logic
/// inside the implementation. It should be used only by the
/// library
pub unsafe trait DeviceData {
    type Ty: Send + Sync;

    fn data(&self) -> &Self::Ty;
    fn data_mut(&mut self) -> &mut Self::Ty;
}

pub trait Device<T>: DeviceData<Ty = T> {
    type Error;

    fn read(
        &self,
        dest: &mut [u8],
        size: u8,
        offset: usize,
    ) -> Result<(), Self::Error>;

    fn write(
        &self,
        dest: &mut [u8],
        size: u8,
        offset: usize,
    ) -> Result<(), Self::Error>;
}
