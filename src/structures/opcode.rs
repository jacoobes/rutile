use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
pub enum OpCode {
    Halt        = 0x000000, //End execution 
    LoadConst   = 0x000001, //load constant
    Negate      = 0x000002, //negate a number
    Add         = 0x000003, // add two values
    Sub         = 0x000004, // subtract two numbers
    Mul         = 0x000005, // multiply two numbers
    Div         = 0x000006, // divide two numbers
    Mod         = 0x000007, // modulus two numbers
    IfLess      = 0x000008, // IfLessThan
    IfGreater   = 0x000009, // IfGreaterThan
    IfEqual     = 0x00000a, // IfEqual
    Not         = 0x00000b, // IfStringEquals
    And         = 0x00000c, // IfBoEq
    Or          = 0x00000d, // Or boolean operator
}
