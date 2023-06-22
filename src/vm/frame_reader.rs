use crate::structures::{value::Value, opcode::OpCode};


//ATM this will just return a vec of values given instructions
//will be adding more to a real vm later
pub fn read_frame() -> Result<Vec<Value>, String> {
    let mut instr_ptr = 0usize;
    let values: Vec<Value> = Vec::new();
    let v = vec![1];
    while let Some(i) = v.get(instr_ptr) {
        let instruction = OpCode::try_from(*i).unwrap();
        match instruction {
            OpCode::LoadConst => {
            }
            OpCode::Halt => return Ok(values),
            OpCode::Negate => {
            },
            OpCode::Not => {
            }
            OpCode::Add =>  {
            }
            OpCode::Sub => {
            },
            OpCode::Mul => {
            },
            OpCode::Div => {
            },
            OpCode::Mod => {
            },
            OpCode::IfEqual => {
            },
            OpCode::IfLess | OpCode::IfGreater => {
            }
            OpCode::And => {
            }
            OpCode::Or => {
            }
            OpCode::DefLocal => {
            }
            OpCode::GetLocal => {
            }
            OpCode::PopN => {
            }
            OpCode::Pop => {
            }
        }
        instr_ptr += 1;
    }
    Err("Program did not halt correctly".to_string())
}

