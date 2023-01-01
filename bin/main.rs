use rvvm::prelude::*;

#[type_handler(ty = "()")]
fn on_reset(dev: &Device<()>) {
    println!("Reset");
}

#[type_handler(ty = "()")]
fn on_update(dev: &Device<()>) {
    println!("Update");
}

#[on_remove(ty = "()")]
fn on_remove(dev: &mut Device<()>) {
    println!("I'm being removed");
}

fn main() {
    // Creates the RVVM instance with the 4KB of memory and 1
    // hart, see `rvvm::builders::instance::InstanceBuilder`
    // documentation for more detailed information or use the
    // `rvvm::instance::Instance::new` for direct creation
    let mut instance = Instance::builder().build();

    let test_type =
        DeviceType::custom("Test", on_remove, on_update, on_reset);
    let test_dev = Device::builder()
        .address(0x1024)
        .size(1024)
        .device_type(test_type)
        .op_size(1..=1) // inclusive range [from; to]
        .data(()) // Data for ZST types will not be allocated, but dropped
        .build();
    let handle: DeviceHandle<()> = instance
        .try_attach_device(test_dev)
        .expect("Failed to attach MMIO device");

    dbg!(handle);
}
