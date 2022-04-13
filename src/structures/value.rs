use smol_str::SmolStr;

#[derive(Debug)]
pub enum Value {
    Str(SmolStr),
    Float(f64),
    Integer(i64),
    Boolean(bool),
    Char(char)
}


impl Value {
    //TODO : Is there a more optimal way? https://doc.rust-lang.org/std/mem/fn.take.html
    pub fn negate(&self) -> Value {
        match self {
            Value::Float(f) => Value::Float(-f),
            Value::Integer(i) => Value::Integer(-i),
            _ => panic!("Cannot negate {:?}", &self)
        }
    }
    
}
