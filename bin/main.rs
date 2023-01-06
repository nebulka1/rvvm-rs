use rvvm::prelude::*;

#[device]
struct TestDev(i32);

impl Drop for TestDev {
    fn drop(&mut self) {
        eprintln!("drop TestDev");
    }
}

impl Device<i32> for TestDev {
    type Error = ();

    fn name() -> &'static str {
        "123"
    }

    fn read(
        &self,
        _dest: &mut [u8],
        _size: u8,
        _offset: usize,
    ) -> Result<(), Self::Error> {
        self.data();
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
    let _a = TestDev::new(10, 20, 1..=1, 1024i32);
    dbg!(TestDev::name());
}
