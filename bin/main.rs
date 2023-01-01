use rvvm::{
    c_str,
    dev::{
        mmio::Device,
        type_::DeviceType,
    },
    instance::*,
};

fn main() {
    let devty = DeviceType::<i32>::new(c_str!("UART"));
    let dev = Device::new(0x1024, 1024, 10, devty, 1..=1);

    let mut instance = Instance::builder().build();
    dbg!(instance.attach_device(dev));
}
