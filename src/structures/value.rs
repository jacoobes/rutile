use bincode::{Encode, Decode};


#[derive(PartialEq, Debug, Encode, Decode, Clone )]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Char(char),
    ObjRef(i32)
}
