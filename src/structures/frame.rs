use std::fmt::Debug;

use super::{value::Value, frame_builder::FrameBuilder, stack::Stack};
use std::rc::Rc;
#[derive(Debug)]
pub struct Frame {
    pub name : String,
    pub bytecode : Stack<u8>,
    pub constants : Vec<Rc<Value>>
}

impl Frame {

    pub fn builder (name : String) -> FrameBuilder {
        FrameBuilder::default(name)
    }

    pub fn top(&self) -> Option<&u8> {
        self.bytecode.top()
    }
}

