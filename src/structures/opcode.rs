use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
pub enum OpCode {
    Halt        = 0x000000,
    LoadConst   = 0x000001,
    Negate      = 0x000002,
    Add         = 0x000003,
    Sub         = 0x000004,
    Mul         = 0x000005,
    Div         = 0x000006,
    Mod         = 0x000007
}
