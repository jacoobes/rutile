
use smol_str::SmolStr;

use super::opcode::OpCode;

#[derive(Debug)]
pub enum Value {
    Str(SmolStr),
    Number(f64),
    Boolean(bool),
    Char(char)
}


impl Value {
    //TODO : Is there a more optimal way? https://doc.rust-lang.org/std/mem/fn.take.html
    pub fn negate(&self) -> Value {
        match self {
            Value::Number(f) => Value::Number(-f),
            _ => panic!("Cannot negate {:?}", &self)
        }
    }
    // Will combine two values into another value. 
    // panics if conversion is impossible
    pub fn combine(lhs : &Value, rhs: &Value, sign : &str) -> Value {
        match (sign, lhs, rhs) {
            // str + str
            ("+", Value::Str(s), Value::Str(o)) => Value::Str(SmolStr::from(format!("{}{}", s,o))), 
            ("+", Value::Char(c), Value::Char(c2)) => Value::Str((c.to_string() + &c2.to_string()).into()),
            // str + char into str
            ("+", Value::Str(_), Value::Char(c)) => Value::combine(lhs, &Value::Str(c.to_string().into()), "+"),
            // float + float
            ("+", Value::Number(f), Value::Number(f2)) => Value::Number(f + f2),
            ("-", Value::Number(_), Value::Number(_)) => Value::combine(lhs, &rhs.negate(), "+"),
            ("/", Value::Number(f), Value::Number(f2)) => Value::Number(f / f2),
            ("*", Value::Number(f), Value::Number(f2)) => Value::Number(f * f2),
            ("%", Value::Number(f), Value::Number(f2)) => Value::Number(f % f2),
            (_sign, _, _) => panic!("Cannot combine these two types : {:?}, {:?} with {_sign}", lhs, rhs)
        }
    }
    pub fn equals(lhs: &Value, rhs: &Value) -> Value {
        match ( lhs, rhs ) {
            (Value::Number(f), Value::Number(f2)) => Value::Boolean(f == f2),
            (Value::Str(s), Value::Str(s2)) => Value::Boolean(s == s2),
            (Value::Boolean(b), Value::Boolean(b2)) => Value::Boolean(b == b2),
            (_, _) => panic!("Cannot compare these two types : {:?}, {:?}", lhs,rhs )
        }
    }
    pub fn cmp(lhs: &Value, rhs: &Value, code : &OpCode) -> Value {
        match( code, lhs, rhs ) {
            (OpCode::IfLess, Value::Number(f), Value::Number(f2)) => Value::Boolean(f < f2),
            (OpCode::IfGreater, Value::Number(f), Value::Number(f2)) => Value::Boolean(f > f2),
            (_, _, _) => panic!("Cannot compare these two types : {:?}, {:?}, with {:?}", lhs,rhs,code)
        }
    }
    pub fn flip(val: &Value) -> Value {
        match val  {
            Value::Boolean(b) => Value::Boolean(*b),
            _ => panic!("Cannot use ! on {:?}", &val)
        }
    }
}
