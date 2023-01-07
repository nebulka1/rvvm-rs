use rvvm::prelude::*;

#[device]
struct Uart(i32);

impl Device<i32> for Uart {
    type Error = ();

    fn name() -> &'static str {
        "UART"
    }

    fn read(
        &self,
        _dest: &mut [u8],
        _size: u8,
        _offset: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(
        &self,
        _dest: &mut [u8],
        _size: u8,
        _offset: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn main() {
    let mut instance = Instance::builder().build();
    let uart = Uart::new(0x1024, 100, 1..=1, 10);

    if let Ok(handle) = instance.try_attach_device(uart) {
        println!("Attached {} at {:?}", Uart::name(), handle);
    } else {
        println!("Failed to attach Uart");
    }
}
