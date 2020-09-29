use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Syscall {
    PrintReg,
    PrintMem,
    PrintStr,
}
