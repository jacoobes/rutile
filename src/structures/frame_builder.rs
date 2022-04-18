
use crate::Frame;
use crate::structures::value::Value;
use crate::OpCode;
use std::rc::Rc;
use super::stack::Stack;

pub struct FrameBuilder {
    name : String,
    bytecode : Stack<u8>,
    constants : Vec<Rc<Value>>
}

impl FrameBuilder {
    
    pub fn new (name : String) -> FrameBuilder {
        FrameBuilder { name, bytecode: Stack::default(), constants: Vec::new() }
    }
    
    pub fn push_const(mut self, value: Rc<Value>) -> FrameBuilder {
        let idx = self.constants.len() as u8;
        self.constants.push(value);
        self.bytecode.push(OpCode::LoadConst.into());
        self.bytecode.push(idx);
        self
    }
    pub fn push_opcode(mut self, value: OpCode) -> FrameBuilder {
        self.bytecode.push(value.into());
        self
    }
    pub fn build(self) -> Frame {
        Frame { 
            name : self.name,
            bytecode: self.bytecode,
            constants: self.constants 
        }
    }
}


