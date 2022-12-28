use crate::{
    c_str,
    utils::*,
};

#[test]
fn test_c_chars_macro() {
    let chars = c_str!("NVME");

    assert_eq!(chars.as_c_str().to_str().unwrap(), "NVME");
}

#[test]
fn test_c_chars_preserve_contents() {
    let chars = CChars::new(b"Hello world\0");
    assert_eq!(chars.as_c_str().to_str().unwrap(), "Hello world");
}

#[test]
fn test_c_chars_nul_check() {
    assert!(CChars::try_new(b"Hello world").is_err());
    assert!(CChars::try_new(b"Hello world\0").is_ok());
}
