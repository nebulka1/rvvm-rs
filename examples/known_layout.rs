use rvvm::{
    macros::KnownLayout,
    types::KnownLayout,
};

#[derive(KnownLayout, Clone, Copy)]
#[repr(C)]
pub struct Test;

#[derive(KnownLayout, Clone, Copy)]
#[repr(u8)]
pub enum A {
    B { v: i32 },
}

fn consume(_: impl KnownLayout) {}

fn main() {
    consume([1; 10]);
    consume([A::B { v: 10 }; 10]);
    consume([Test; 20]);
    consume(Test);
    consume(A::B { v: 30 });
}
