use rvvm::{
    c_str,
    fdt::*,
};

fn main() {
    let mut root = NodeBuf::root();
    root.add(c_str!("Nero"), 32u32);

    dbg!(root.find("Nero").is_some());
}
