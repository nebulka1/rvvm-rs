# rvvm [WIP]

Safe Rust bindings to the RVVM's public API.

Provides the Rust-idiomatic interface to the [RVVM](https://github.com/lekkit/rvvm) public API.

# Example

```rust
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

```

# Implemented

- [x] Virtual machine creation
- [x] RAM read/writes
- [x] Sound (in terms of safety) and type-safe api for devices
  - [ ] Complete API (needs rechecking)
- [x] Flattened device tree library bindings
- [x] Access to the virtual machine's FDT
- [x] `pause`/`start`/`powered_on` APIs

- [x] loading kernel/bootrom/dtb
- [ ] Run virtual machine's event loop
- [ ] PLIC/i2c
- [ ] Userland API
- [ ] `DMA`
- [ ] Test `UART` device
- [ ] rvvm-sys features (like fdt, jit or smth)

Possibly I've lost something, so the list will be supplemented/updated. 
