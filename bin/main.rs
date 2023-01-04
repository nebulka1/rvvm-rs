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

    fn read(
        &self,
        dest: &mut [u8],
        size: u8,
        offset: usize,
    ) -> Result<(), Self::Error> {
        self.data();
        Ok(())
    }

    fn write(
        &self,
        dest: &mut [u8],
        size: u8,
        offset: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn main() {
    let a = TestDev::new(10, 20, 1..=1, 1024i32);
}
