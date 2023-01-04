use rvvm::{
    ffi,
    prelude::*,
};
use rvvm_macro::DeviceTypeExt;

#[rvvm_macro::device]
struct Uart(());

#[derive(DeviceTypeExt)]
struct UartType(ffi::rvvm_mmio_type_t);

impl DeviceType for UartType {
    type Device = Uart;

    fn name(&self) -> &std::ffi::CStr {
        std::ffi::CStr::from_bytes_with_nul(b"uart\0").unwrap()
    }

    fn remove(_dev: &mut Self::Device) {
        println!("UART removed");
    }

    fn reset(_dev: &mut Self::Device) {
        println!("UART reset");
    }

    fn update(_dev: &mut Self::Device) {
        println!("UART updated");
    }
}

impl Device<()> for Uart {
    type Error = ();

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
    // Creates the RVVM instance with the 4KB of memory and 1
    // hart, see `rvvm::builders::instance::InstanceBuilder`
    // documentation for more detailed information or use the
    // `rvvm::instance::Instance::new` for direct creation
    let mut instance = Instance::builder().build();

    let test_type = UartType::new();
    let test_dev = rvvm_sys::rvvm_mmio_dev_t {
        addr: 0x1000,
        size: 0x1000,
        data: std::ptr::null_mut(),
        machine: std::ptr::null_mut(),
        type_: &test_type.0,
        read: None,
        write: None,
        min_op_size: 1,
        max_op_size: 1,
    };
    let handle = instance
        .try_attach_device(Uart { dev: test_dev })
        .expect("Failed to attach MMIO device");

    dbg!(handle);
}
