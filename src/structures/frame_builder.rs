use super::frame::Frame;
use crate::structures::value::Value;
use super::opcode::OpCode;
use std::rc::Rc;
use smol_str::SmolStr;
use crate::structures::locals::LocalChart;
use crate::structures::tokens::Token;
use super::stack::Stack;

pub struct FrameBuilder {
    name : SmolStr,
    bytecode : Stack<u8>,
    constants : Vec<Rc<Value>>,
    local_chart : LocalChart
}

impl FrameBuilder {

    pub fn new (name : SmolStr) -> FrameBuilder {
        FrameBuilder { name, bytecode: Stack::default(), constants: Vec::new(), local_chart: Default::default() }
    }

    pub fn with_const(&mut self, value: Rc<Value>) -> &mut FrameBuilder {
        let idx = self.constants.len() as u8;
        self.constants.push(value);
        self.bytecode.push(OpCode::LoadConst.into());
        self.with_byte(idx);
        self
    }
    pub fn with_opcode(&mut self, value: OpCode) -> &mut FrameBuilder {
        self.bytecode.push(value.into());
        self
    }
    pub fn with_byte(&mut self, value : u8) -> &mut FrameBuilder {
        self.bytecode.push(value);
        self
    }
    pub fn with_def_local(&mut self, token : Token) -> u8 {
        let idx = self.local_chart.locals().len() as u8;
        self.local_chart.new_local(token);
        self.with_opcode(OpCode::DefLocal);
        idx
    }
    pub fn new_scope(&mut self) {
        self.local_chart.inc_depth()
    }

    pub fn resolve_local(&mut self, name : &Token) -> u8 {
        self.local_chart.resolve_local(name).expect("Expected to find some local, found none") as u8
    }
    pub fn leave_scope(&mut self) {
        let amt_dropped = self.local_chart.dec_depth();
        let idx = self.constants.len() as u8;
        self.with_byte(amt_dropped as u8);
        self.with_opcode(OpCode::PopN);
        self.with_byte(idx);
    }

    pub fn build(self) -> Frame {
        Frame {
            name : self.name,
            bytecode: self.bytecode,
            constants: self.constants,
            local_chart: self.local_chart
        }
    }
}
