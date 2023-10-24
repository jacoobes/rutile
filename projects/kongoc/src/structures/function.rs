pub struct Function {
    pub name: String,
    pub arity: u8,
    bytecode: Vec<u8>
}

impl Function {
    pub fn new(name: String, arity: u8, bytecode: Vec<u8>) -> Self {
        Function {
            name,
            arity,
            bytecode
        }
    }
}
