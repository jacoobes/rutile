pub mod structures;
pub mod vm;

use structures::{ frame::*, opcode::*, value::Value::* };
use std::rc::Rc;
fn main() {
    let frame = Frame::builder(String::from("fn1")) 
        .push_const(Rc::new(Number(100f64)))
        .push_const(Rc::new(Number(2f64)))
        .push_opcode(OpCode::Sub)
        .push_opcode(OpCode::Halt)
        .build();

    println!("{:?}", &frame);
    let data = vm::frame_reader::read_frame(frame).unwrap();
    println!("{:?}", data);
}


