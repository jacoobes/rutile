use crate::structures::{opcode::OpCode, bytecode_file::BytecodeFile, value::Value, locals::LocalChart};


//ATM this will just return a vec of values given instructions
//will be adding more to a real vm later
pub fn interpret_unit(bc_file: BytecodeFile, local_chart: LocalChart) -> Result<(), String> {
    let mut instr_ptr = 0usize;
    let mut values = Vec::<Value>::new();
    while let Some(i) = bc_file.get_byte(instr_ptr) {
        let instruction = OpCode::try_from(i).unwrap();
        match instruction {
            // pushing constants into the stack. 
            OpCode::LoadConst => {
                instr_ptr += 1;
                let value = bc_file.get_byte(instr_ptr)
                    .and_then(|byte| bc_file.get_const(byte as usize))
                    .expect("LoadConst: Expected index to be at location");
                values.push(value.clone());
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
                let last = values.pop();
                if let Some(Value::Boolean(x)) = last {
                    values.push(Value::Boolean(!x))    
                } else {
                    panic!("Tried to negate non number")
                }
            }
            OpCode::Add =>  {
                let o1 = values.pop().expect("Expected el");
                let o2 = values.pop().expect("Expected el");
                values.push(o1 + o2);
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

            },
        }
        instr_ptr += 1;
    }
    Err("Program did not halt correctly".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(5, 5);
    }
}
