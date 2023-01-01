use std::ops::RangeInclusive;

use crate::prelude::{
    Device,
    DeviceType,
    RwHandler,
};

pub struct DeviceBuilder<T> {
    address: Option<u64>,
    size: Option<usize>,

    data: Option<T>,
    type_: Option<DeviceType<T>>,

    read: RwHandler<T>,
    write: RwHandler<T>,

    op_sizes: Option<RangeInclusive<u8>>,
}

impl<T> Default for DeviceBuilder<T> {
    fn default() -> Self {
        Self {
            address: None,
            size: None,
            data: None,
            type_: None,
            read: RwHandler::none(),
            write: RwHandler::none(),
            op_sizes: None,
        }
    }
}

impl<T> DeviceBuilder<T> {
    pub fn device_type(self, ty: DeviceType<T>) -> Self {
        self.type_(ty)
    }

    pub fn type_(mut self, ty: DeviceType<T>) -> Self {
        self.type_ = Some(ty);
        self
    }

    pub fn op_size(mut self, range: RangeInclusive<u8>) -> Self {
        self.op_sizes = Some(range);
        self
    }

    pub fn write(mut self, write: RwHandler<T>) -> Self {
        self.write = write;
        self
    }

    pub fn read(mut self, read: RwHandler<T>) -> Self {
        self.read = read;
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size.into();
        self
    }

    pub fn address(mut self, addr: u64) -> Self {
        self.address = addr.into();
        self
    }

    pub fn build(self) -> Device<T> {
        Device::new(
            self.address
                .expect("Device address is not specified"),
            self.size.expect("Size is not specified"),
            self.data.expect("Data is not specified"),
            self.type_.expect("Device type is not specified"),
            self.read,
            self.write,
            self.op_sizes.expect("op sizes is not specified"),
        )
    }
}
