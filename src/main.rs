pub mod structures;
pub mod vm;

use smol_str::SmolStr;
use structures::{ frame::*, opcode::*, value::Value::* };
use std::rc::Rc;
fn main() {
    let frame = Frame::builder(String::from("fn1")) 
        .push_const(Rc::new(Integer(10)), 0)
        .push_opcode(OpCode::Negate)
        .push_opcode(OpCode::Halt)
        .build();

    println!("{:?}", &frame);
    let data = vm::frame_reader::read_frame(frame).unwrap();
    println!("{:?}", data);
}


