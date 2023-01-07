use integral_enum::IntegralEnum;
use rvvm_sys::{
    RVVM_REGID_CAUSE,
    RVVM_REGID_F0,
    RVVM_REGID_PC,
    RVVM_REGID_TVAL,
    RVVM_REGID_X0,
};

#[derive(IntegralEnum)]
#[repr(usize)]
pub enum Register {
    X0 = RVVM_REGID_X0 as usize,
    X1 = RVVM_REGID_X0 as usize + 1,
    X2 = RVVM_REGID_X0 as usize + 2,
    X3 = RVVM_REGID_X0 as usize + 3,
    X4 = RVVM_REGID_X0 as usize + 4,
    X5 = RVVM_REGID_X0 as usize + 5,
    X6 = RVVM_REGID_X0 as usize + 6,
    X7 = RVVM_REGID_X0 as usize + 7,
    X8 = RVVM_REGID_X0 as usize + 8,
    X9 = RVVM_REGID_X0 as usize + 9,
    X10 = RVVM_REGID_X0 as usize + 10,
    X11 = RVVM_REGID_X0 as usize + 11,
    X12 = RVVM_REGID_X0 as usize + 12,
    X13 = RVVM_REGID_X0 as usize + 13,
    X14 = RVVM_REGID_X0 as usize + 14,
    X15 = RVVM_REGID_X0 as usize + 15,
    X16 = RVVM_REGID_X0 as usize + 16,
    X17 = RVVM_REGID_X0 as usize + 17,
    X18 = RVVM_REGID_X0 as usize + 18,
    X19 = RVVM_REGID_X0 as usize + 19,
    X20 = RVVM_REGID_X0 as usize + 20,
    X21 = RVVM_REGID_X0 as usize + 21,
    X22 = RVVM_REGID_X0 as usize + 22,
    X23 = RVVM_REGID_X0 as usize + 23,
    X24 = RVVM_REGID_X0 as usize + 24,
    X25 = RVVM_REGID_X0 as usize + 25,
    X26 = RVVM_REGID_X0 as usize + 26,
    X27 = RVVM_REGID_X0 as usize + 27,
    X28 = RVVM_REGID_X0 as usize + 28,
    X29 = RVVM_REGID_X0 as usize + 29,
    X30 = RVVM_REGID_X0 as usize + 30,
    X31 = RVVM_REGID_X0 as usize + 31,

    F0 = RVVM_REGID_F0 as usize,
    F1 = RVVM_REGID_F0 as usize + 1,
    F2 = RVVM_REGID_F0 as usize + 2,
    F3 = RVVM_REGID_F0 as usize + 3,
    F4 = RVVM_REGID_F0 as usize + 4,
    F5 = RVVM_REGID_F0 as usize + 5,
    F6 = RVVM_REGID_F0 as usize + 6,
    F7 = RVVM_REGID_F0 as usize + 7,
    F8 = RVVM_REGID_F0 as usize + 8,
    F9 = RVVM_REGID_F0 as usize + 9,
    F10 = RVVM_REGID_F0 as usize + 10,
    F11 = RVVM_REGID_F0 as usize + 11,
    F12 = RVVM_REGID_F0 as usize + 12,
    F13 = RVVM_REGID_F0 as usize + 13,
    F14 = RVVM_REGID_F0 as usize + 14,
    F15 = RVVM_REGID_F0 as usize + 15,
    F16 = RVVM_REGID_F0 as usize + 16,
    F17 = RVVM_REGID_F0 as usize + 17,
    F18 = RVVM_REGID_F0 as usize + 18,
    F19 = RVVM_REGID_F0 as usize + 19,
    F20 = RVVM_REGID_F0 as usize + 20,
    F21 = RVVM_REGID_F0 as usize + 21,
    F22 = RVVM_REGID_F0 as usize + 22,
    F23 = RVVM_REGID_F0 as usize + 23,
    F24 = RVVM_REGID_F0 as usize + 24,
    F25 = RVVM_REGID_F0 as usize + 25,
    F26 = RVVM_REGID_F0 as usize + 26,
    F27 = RVVM_REGID_F0 as usize + 27,
    F28 = RVVM_REGID_F0 as usize + 28,
    F29 = RVVM_REGID_F0 as usize + 29,
    F30 = RVVM_REGID_F0 as usize + 30,
    F31 = RVVM_REGID_F0 as usize + 31,

    PC = RVVM_REGID_PC as usize,
    CAUSE = RVVM_REGID_CAUSE as usize,
    TVAL = RVVM_REGID_TVAL as usize,
}

#[rustfmt::skip] // reorder_impl_items screws the order
impl Register {
    pub const ZERO: Register = Register::X0;
    pub const RA: Register = Register::X1;
    pub const SP: Register = Register::X2;
    pub const GP: Register = Register::X3;
    pub const TP: Register = Register::X4;
    pub const T0: Register = Register::X5;
    pub const T1: Register = Register::X6;
    pub const T2: Register = Register::X7;
    pub const S0: Register = Register::X8;
    pub const FP: Register = Register::X8; // same as S0
    pub const S1: Register = Register::X9;
    pub const A0: Register = Register::X10;
    pub const A1: Register = Register::X11;
    pub const A2: Register = Register::X12;
    pub const A3: Register = Register::X13;
    pub const A4: Register = Register::X14;
    pub const A5: Register = Register::X15;
    pub const A6: Register = Register::X16;
    pub const A7: Register = Register::X17;
    pub const S2: Register = Register::X18;
    pub const S3: Register = Register::X19;
    pub const S4: Register = Register::X20;
    pub const S5: Register = Register::X21;
    pub const S6: Register = Register::X22;
    pub const S7: Register = Register::X23;
    pub const S8: Register = Register::X24;
    pub const S9: Register = Register::X25;
    pub const S10: Register = Register::X26;
    pub const S11: Register = Register::X27;
    pub const T3: Register = Register::X28;
    pub const T4: Register = Register::X29;
    pub const T5: Register = Register::X30;
    pub const T6: Register = Register::X31;
    pub const FT0: Register = Register::F0;
    pub const FT1: Register = Register::F1;
    pub const FT2: Register = Register::F2;
    pub const FT3: Register = Register::F3;
    pub const FT4: Register = Register::F4;
    pub const FT5: Register = Register::F5;
    pub const FT6: Register = Register::F6;
    pub const FT7: Register = Register::F7;
    pub const FS0: Register = Register::F8;
    pub const FS1: Register = Register::F9;
    pub const FA0: Register = Register::F10;
    pub const FA1: Register = Register::F11;
    pub const FA2: Register = Register::F12;
    pub const FA3: Register = Register::F13;
    pub const FA4: Register = Register::F14;
    pub const FA5: Register = Register::F15;
    pub const FA6: Register = Register::F16;
    pub const FA7: Register = Register::F17;
    pub const FS2: Register = Register::F18;
    pub const FS3: Register = Register::F19;
    pub const FS4: Register = Register::F20;
    pub const FS5: Register = Register::F21;
    pub const FS6: Register = Register::F22;
    pub const FS7: Register = Register::F23;
    pub const FS8: Register = Register::F24;
    pub const FS9: Register = Register::F25;
    pub const FS10: Register = Register::F26;
    pub const FS11: Register = Register::F27;
    pub const FT8: Register = Register::F28;
    pub const FT9: Register = Register::F29;
    pub const FT10: Register = Register::F30;
    pub const FT11: Register = Register::F31;
}
