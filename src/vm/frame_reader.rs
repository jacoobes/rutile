use std::rc::Rc;

use crate::structures::{value::Value, frame::Frame, opcode::OpCode, stack::Stack};


//ATM this will just return a vec of values given instructions
//will be adding more to a real vm later
pub fn read_frame(frame : Frame) -> Result<Stack<Rc<Value>>, String> {
    let mut instr_ptr = 0usize;
    let mut values: Stack<Rc<Value>> = Stack::default();
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
                    None => panic!("Expected a constant in pool near {i}")
                }
            }
            OpCode::Halt => return Ok(values),
            OpCode::Negate => {
                if let Some(val) = values.pop()
                .map(| val | { 
                    Rc::new(Value::negate(&val))
                }) { values.push(val) }
                else { panic!("Expected a value to negate, found none") }
            },
            OpCode::Not => {
                if let Some(val) = values.pop()
                .map(| val | { 
                    Rc::new(Value::flip(&val))
                }) { values.push(val) }
                else { panic!("Expected a value to flip, found none") }
            }
            OpCode::Add =>  {
                let [rhs, lhs] = pop_two(&mut values, "Tried popping stack +, nothing found!");
                values.push(Rc::new(Value::combine(&lhs, &rhs, "+")))
            }
            OpCode::Sub => {
                let [rhs, lhs] = pop_two(&mut values, "Tried popping stack -, nothing found!");
                values.push(Rc::new(Value::combine(&lhs, &rhs, "-")))
            },
            OpCode::Mul => {
                let [rhs, lhs] = pop_two(&mut values, "Tried popping stack *, nothing found!");
                values.push(Rc::new(Value::combine(&lhs, &rhs, "*")))
            },
            OpCode::Div => {
                let [rhs,lhs] = pop_two(&mut values, "Tried popping stack /; nothing found!");
                values.push(Rc::new(Value::combine(&lhs, &rhs, "/")))
            },
            OpCode::Mod => {
                let [rhs,lhs] = pop_two(&mut values, "Tried popping stack %; nothing found!");
                values.push(Rc::new(Value::combine(&lhs, &rhs, "%")))
            },
            OpCode::IfNuEq | OpCode::IfStEq | OpCode::IfBoEq => {
                let [rhs, lhs] = pop_two(&mut values, "Tried popping stack ==; nothing found!");
                values.push(Rc::new(Value::equals(&lhs, &rhs, &instruction)))
            },
            OpCode::IfNuLs | OpCode::IfNuGr => {
                let [rhs, lhs] = pop_two(&mut values, "Tried popping stack < >; nothing found!");
                values.push(Rc::new(Value::cmp(&lhs, &rhs, &instruction)))
            }
        }
        instr_ptr += 1;
    }
    Err("Program did not halt correctly".to_string())
}

fn pop_two(vec: &mut Stack<Rc<Value>>, msg: &'static str) -> [Rc<Value>;2] {
    [vec.pop().expect(msg), vec.pop().expect(msg)]
}

