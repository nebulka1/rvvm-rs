use rvvm::prelude::*;

fn main() {
    let mut instance = Instance::builder().build();
    let uart_ty = DeviceType::new("UART");
    let uart = Device::builder()
        .address(0x1024)
        .size(1024)
        .data(())
        .device_type(uart_ty)
        .op_size(1..=1)
        .build();

    match instance.attach_device(uart) {
        Ok(handle) => {
            println!("Attached UART: {handle:?}");
        }

        Err(e) => {
            eprintln!("Failed to attach UART: {e}");
        }
    }
}
