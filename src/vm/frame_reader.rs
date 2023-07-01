use crate::structures::{opcode::OpCode, bytecode_file::BytecodeFile, value::Value};


//ATM this will just return a vec of values given instructions
//will be adding more to a real vm later
pub fn interpret_unit(c: BytecodeFile) -> Result<(), String> {
    let mut instr_ptr = 0usize;
    let mut values = Vec::<Value>::new();
    while let Some(i) = c.get(instr_ptr) {
        let instruction = OpCode::try_from(*i).unwrap();
        match instruction {
            OpCode::LoadConst => {

            }
            OpCode::Halt => return Ok(()),
            OpCode::Negate => {
                let last = values.pop();
                if let Some(Value::Number(x)) = last {
                    values.push(Value::Number(-x))    
                } else {
                    panic!("Tried to negate non number")
                }
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
            OpCode::IfLess => {
            }
            OpCode::IfGreater => {
            }
            OpCode::And => {
            }
            OpCode::Or => {
            }
            OpCode::StoreConst => { 
                instr_ptr += 1;
                let idx = c.get(instr_ptr).expect("StoreConst: expected index");
                let data = c.get_const(*idx as usize).unwrap();
                //for now clone
                values.push(data.clone())
            },
        }
        instr_ptr += 1;
    }
    Err("Program did not halt correctly".to_string())
}

