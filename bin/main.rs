use rvvm::{
    c_str,
    dest_ptr::DestPtr,
    dev::{
        mmio::{
            DeviceDescriptor,
            MmioHandler,
        },
        mmio_type::MmioType,
    },
    instance::Instance,
};
use rvvm_macro::rw_handler;

#[rw_handler("()")]
fn write(
    dev: DeviceDescriptor<'_, ()>,
    dest: DestPtr<'_>,
) -> Result<(), ()> {
    Ok(())
}

#[rw_handler("()")]
fn read(
    dev: DeviceDescriptor<'_, ()>,
    dest: DestPtr<'_>,
) -> Result<(), ()> {
    Ok(())
}

fn main() {
    let mut instance =
        Instance::new(Instance::DEFAULT_BASE_ADDRESS, 4096, 1, true)
            .unwrap();
    let dev = DeviceDescriptor::new(
        0x1024,
        12,
        1..=1,
        MmioHandler::new(read),
        MmioHandler::new(write),
        (),
        MmioType::new(c_str!("Fucker"), None, None, None),
    );
    let res = instance.attach_device(dev);

    dbg!(res.unwrap());
}
