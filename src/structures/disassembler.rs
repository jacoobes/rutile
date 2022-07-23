use crate::structures::frame::Frame;
use crate::structures::opcode::OpCode;

pub struct Disassembler;

impl Disassembler {
    pub fn disassemble(frame: Frame) {
        let base_str = format!("{:-^40}", &frame.name);
        println!("{}", base_str);
        println!("Instruction{:^23}{:>10}\n\n","Hex", "Constant");
        let mut offset = 0usize;
        while offset < frame.bytecode.len() {
            offset = Disassembler::disassemble_instruction(&frame, &mut offset)
        }

        println!("{:-^40}","-")
    }

    fn disassemble_instruction(frame : &Frame, offset : &mut usize) -> usize {
        let instruction = frame.bytecode.get(*offset).expect("expected an opcode");
        OpCode::try_from(*instruction).map(| op | match op {
                    OpCode::Halt => Disassembler::simple_instruction(offset, op),
                    OpCode::LoadConst => Disassembler::const_instruction(frame, offset, op),
                    OpCode::Mul
                    | OpCode::Negate
                    | OpCode::Add
                    | OpCode::Sub
                    | OpCode::Div
                    | OpCode::Mod
                    | OpCode::IfLess
                    | OpCode::IfGreater
                    | OpCode::IfEqual
                    | OpCode::Not
                    | OpCode::And
                    | OpCode::Or
                    | OpCode::Pop => Disassembler::simple_instruction(offset, op),
                    OpCode::PopN
                    | OpCode::DefLocal => Disassembler::pop_instruction(frame, offset, op)
      }).unwrap()
    }
    fn simple_instruction(offset: &mut usize, instruction: OpCode) -> usize {
        println!(
            "{:<20}{:#04x}",
            format!("{:?}", &instruction),
            <OpCode as Into<u8>>::into(instruction),
        );
        *offset + 1
    }

    fn const_instruction(chunk: &Frame, offset: &mut usize, instruction: OpCode) -> usize {
        let loc_of_const = chunk.bytecode[*offset + 1] as usize;
        print!(
            "{:<20}{:#04x}",
            format!("{:?}", &instruction),
            <OpCode as Into<u8>>::into(instruction),
        );
        print!("{:>10}", &loc_of_const);
        println!(" {:?}", chunk.constants[loc_of_const]);
        *offset + 2
    }
    fn pop_instruction(chunk: &Frame, offset: &mut usize, instruction: OpCode) -> usize {
        let pop_n = chunk.bytecode[*offset + 1] as usize;
        println!(
            "{:<20}{:#04x}",
            format!("{:?} - {pop_n:?}", &instruction),
            <OpCode as Into<u8>>::into(instruction),
        );
        *offset + 2
    }
}

