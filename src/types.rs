use std::{
    fmt::Debug,
    marker::PhantomData,
};

pub struct DeviceHandle<T> {
    pub(crate) inner: i32,
    phantom: PhantomData<T>,
}

impl<T> Debug for DeviceHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DeviceHandle")
            .field(&self.inner)
            .finish()
    }
}

impl<T> Copy for DeviceHandle<T> {}
impl<T> Clone for DeviceHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}
