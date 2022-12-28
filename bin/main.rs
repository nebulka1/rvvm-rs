use rvvm::{
    c_str,
    dev::{
        mmio::DeviceDescriptor,
        mmio_type::MmioType,
    },
    instance::Instance,
};

fn main() {
    let mut instance =
        Instance::new(Instance::DEFAULT_BASE_ADDRESS, 4096, 1, true)
            .unwrap();
    let dev = DeviceDescriptor::new(
        0x1024,
        12,
        1..=1,
        None,
        None,
        (),
        MmioType::new(c_str!("Fucker"), None, None, None),
    );
    let res = instance.attach_device(dev);

    dbg!(res.unwrap());
}
