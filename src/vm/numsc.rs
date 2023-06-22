use crate::structures::frame::Frame;

pub struct NumsC {
    pub stack_frame : Vec<Frame>
}

impl NumsC {

    pub fn add_frame(&mut self, frame : Frame) {
        self.stack_frame.push(frame)
    }
}
