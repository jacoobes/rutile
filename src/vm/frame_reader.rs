use std::rc::Rc;

use crate::structures::{value::Value, frame::Frame, opcode::OpCode, stack::Stack};


//ATM this will just return a vec of values given instructions
//will be adding more to a real vm later
pub fn read_frame(frame : Frame) -> Result<Stack<Rc<Value>>, ()> {
    let mut instr_ptr = 0usize;
    let mut values: Stack<Rc<Value>> = Stack::new();
    while let Some(i) = frame.bytecode.get(instr_ptr) {
        let instruction = OpCode::try_from(*i).unwrap();
        match instruction {
            OpCode::LoadConst => {
                let const_data = frame.bytecode
                     .get(instr_ptr + 1)
                     .and_then(|data_idx| frame.constants.get(*data_idx as usize));
                instr_ptr += 1;
                match const_data {
                    Some( val ) => values.push(val.clone()),
                    None => panic!("Expected a constant in pool near {i}",)
                }
            }
            OpCode::Halt => return Ok(values),
            OpCode::Negate => {
                if let Some(val) = values.pop()
                .map(| val | { 
                    Rc::new(Value::negate(&val))
                }) { values.push(val) }
            },
            OpCode::Add =>  {
                
            },
            OpCode::Sub => (),
            OpCode::Mul => (),
            OpCode::Div => (),
        }
        instr_ptr += 1;
    }
    Err(())

}




