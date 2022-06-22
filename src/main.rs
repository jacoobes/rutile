pub mod structures;
pub mod vm;

use structures::{ opcode::*, value::Value::* };
use std::rc::Rc;
use crate::structures::frame_builder::FrameBuilder;

fn main() {
    // let builder =  &mut FrameBuilder::new("fn1".into());
    // t(builder);
    // let frame = builder.build();
    // let data = vm::frame_reader::read_frame(frame).unwrap();
    // println!("{:?}", data);
}


fn t (f : &mut FrameBuilder) {
    // f.with_const(Rc::new(Str("hello".into())))
    //     .with_const(Rc::new(Str("hello".into())))
    //     .with_opcode(OpCode::IfStEq)
    //     .with_opcode(OpCode::Halt);

}