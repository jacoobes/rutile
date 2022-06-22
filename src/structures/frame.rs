use std::fmt::Debug;

use super::{value::Value, frame_builder::FrameBuilder, stack::Stack};
use std::rc::Rc;
use smol_str::SmolStr;

#[derive(Debug)]
pub struct Frame {
    pub name : SmolStr,
    pub bytecode : Stack<u8>,
    pub constants : Vec<Rc<Value>>,
}

impl Frame {

    pub fn builder (name : SmolStr) -> FrameBuilder {
        FrameBuilder::new(name)
    }

    pub fn top(&self) -> Option<&u8> {
        self.bytecode.top()
    }
}

