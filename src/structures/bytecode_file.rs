use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::rc::Rc;

use super::value::Value;
//numsc compilation unit is a file
//anything that is included is part of the compilation unit.
//usually the compilation unit has an entry point (main)

pub struct BytecodeFile {
    bytecode: Vec<u8>,
    consts: Vec<Value>,
    pub version: String,
}

impl BytecodeFile {
    pub fn new(file_path: String) -> BytecodeFile {
        let mut buf_reader = File::open(&file_path)
            .map(|file| BufReader::new(file))
            .expect(stringify!("No file found ", file_path));

        let mut vers = [0; 3];

        buf_reader
            .read_exact(&mut vers)
            .unwrap();

        let mut bytecode = Vec::new();

        let mut len = [0;1];
        buf_reader.read(&mut len)
            .unwrap();
        let mut const_section = Vec::<u8>::with_capacity((*len.first().expect("Expected a length for const table")).into());
        buf_reader.read_exact(&mut const_section).unwrap();
        let mut consts = Vec::new();    
        
        for _ in 0..const_section.len() {
           let mut chunk = [0; 16]; 
           buf_reader.read_exact(&mut chunk).expect("Segfault: misaligned chunk of const data");
           let value = bincode::decode_from_slice::<Value, _>(&chunk, bincode::config::standard());
           consts.push(value.unwrap().0);
        }

        buf_reader.read_to_end(&mut bytecode)
            .unwrap();
        
        BytecodeFile {
            bytecode,
            consts,
            version: match std::str::from_utf8(&vers) {
                Ok(v) => String::from(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            }
        }
    }

    pub fn get_byte(&self, index: usize) -> Option<u8> {
        self.bytecode.get(index)
            .map(|o| *o)
    }

    pub fn get_const(&self, idx: usize) -> Option<&Value> {
        self.consts.get(idx)
    }


}

