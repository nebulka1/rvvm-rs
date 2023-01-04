use rvvm::{
    dev::ext::*,
    macros::device,
};

#[device]
struct TestDev(i32);

impl Drop for TestDev {
    fn drop(&mut self) {
        eprintln!("Idu nahui");
    }
}

fn main() {
    let a = TestDev::new(10, 20, 1..=1, 1024i32);
}
