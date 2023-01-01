use rvvm::prelude::*;

// #[on_remove(ty = "()")]
// fn test(dev: &mut Device<()>) {
//     println!("I AM REMOVED!!!");
// }

fn main() {
    let mut instance = Instance::builder().build();
    let uart_ty = DeviceType::custom(
        "UART",
        RemoveHandler::none(),
        TypeHandler::none(),
        TypeHandler::none(),
    );
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
