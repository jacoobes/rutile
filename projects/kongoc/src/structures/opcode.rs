use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
pub enum OpCode {
    Halt        , // End execution 
    LoadConst   , // load constant
    Negate      , // negate a number
    Add         , // add two values
    Sub         , // subtract two numbers
    Mul         , // multiply two numbers
    Div         , // divide two numbers
    Mod         , // modulus two numbers
    IfLess      , // IfLessThan
    IfGreater   , // IfGreaterThan
    IfEqual     , // IfEqual
    Not         , // IfStringEquals
    And         , // IfBoEq
    Or          , // Or boolean operator
    StoreConst  , // Store constant data, has operand for the data

}
