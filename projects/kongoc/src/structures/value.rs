use std::ops::Add;

use bincode::{Encode, Decode};


#[derive(PartialEq, Debug, Encode, Decode, Clone )]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Str(String),
}


impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self ,rhs) {
            (Value::Number(f), Value::Number(f2)) => Value::Number(f + f2),
            (Value::Str(s), Value::Str(s2)) => Value::Str(s+&s2),
            (Value::Number(_), _) => panic!("Unaddable"),
            _ => panic!("Unaddable elements")
        }
    }
}

