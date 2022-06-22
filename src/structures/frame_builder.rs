use super::frame::Frame;
use crate::structures::value::Value;
use super::opcode::OpCode;
use std::rc::Rc;
use smol_str::SmolStr;
use super::stack::Stack;

pub struct FrameBuilder {
    name : SmolStr,
    bytecode : Stack<u8>,
    constants : Vec<Rc<Value>>,
}

impl FrameBuilder {
    
    pub fn new (name : SmolStr) -> FrameBuilder {
        FrameBuilder { name, bytecode: Stack::default(), constants: Vec::new() }
    }
    
    pub fn with_const(&mut self, value: Rc<Value>) -> &mut FrameBuilder {
        let idx = self.constants.len() as u8;
        self.constants.push(value);
        self.bytecode.push(OpCode::LoadConst.into());
        self.bytecode.push(idx);
        self
    }
    pub fn with_opcode(&mut self, value: OpCode) -> &mut FrameBuilder {
        self.bytecode.push(value.into());
        self
    }

    pub fn def_local(&mut self) {

    }

    pub fn build(self) -> Frame {
        Frame { 
            name : self.name,
            bytecode: self.bytecode,
            constants: self.constants 
        }
    }
}


