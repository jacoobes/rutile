use crate::structures::frame::Frame;
use crate::structures::stack::Stack;

pub struct NumsC {
    pub stack_frame : Stack<Frame>
}

impl NumsC {

    pub fn add_frame(&mut self, frame : Frame) {
        self.stack_frame.push(frame)
    }
}