use crate::{
    c_str,
    dev::{
        mmio::*,
        mmio_type::*,
    },
    instance::*,
};

#[test]
fn test_mmio_attach() {
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

    assert!(instance.attach_device(dev).is_ok());
}