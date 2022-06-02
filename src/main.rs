pub mod structures;
pub mod vm;

use structures::{ frame::*, opcode::*, value::Value::* };
use std::rc::Rc;
fn main() {
    let frame = Frame::builder("fn1".into())
        .push_const(Rc::new(Str("hello".into())))
        .push_const(Rc::new(Str("hello".into())))
        .push_opcode(OpCode::IfStEq)
        .push_opcode(OpCode::Halt)
        .build();

    println!("{:?}", &frame);
    let data = vm::frame_reader::read_frame(frame).unwrap();
    println!("{:?}", data);
}


