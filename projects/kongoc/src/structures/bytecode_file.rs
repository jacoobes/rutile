use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
use std::io::Seek;
use std::io::BufReader;

use super::function::Function;
use super::value::Value;
//numsc compilation unit is a file
//anything that is included is part of the compilation unit.
//usually the compilation unit has an entry point (main)

pub struct BytecodeFile {
    bytecode: Vec<u8>,
    functions: HashMap<String, Function>,
    consts: Vec<Value>,
    pub version: String,
}

//impl From<Vec<u8>> for BytecodeFile {
//
//    fn from(values: Vec<u8>) -> Self {
//        let version  = &values[0..3];
//        let len = values.get(4).expect("Expected an byte in sequence");
//        
//        let mut consts = Vec::with_capacity(*len as usize);
//        
//        for _ in 0..*len {
//            let mut chunk: Vec<u8> = vec![];
//            //This is where the bincode discriminator. It will be 4 bytes 
//            let mut discrim = [0u8; 4]; 
//            //We read it here^
//            buf_reader
//                .read_exact(&mut discrim)
//                .expect("Segfault: misaligned chunk of const data");
//            //We add the discrim to the slice
//            chunk.extend_from_slice(&discrim);
//            match u32::from_le_bytes(discrim) {
//                0 => {
//                    let mut num = [0u8; 8];
//                    buf_reader.read_exact(&mut num).expect("Segfault: could not find number value after discrim");
//                    chunk.extend_from_slice(&num);
//                    &chunk
//                }
//                1 => { 
//                    let mut bool = [0u8; 1];
//                    buf_reader.read_exact(&mut bool).expect("Segfault: could not find boolean value after discrim");
//                    chunk.push(*bool.first().unwrap());
//                    &chunk
//                }
//                2 => {
//                  let mut len = [0u8; 8];
//                  buf_reader.read_exact(&mut len).expect("Segfault: missing chunk of data for string length");
//                  chunk.extend_from_slice(&len);
//                  let mut str_data = vec![0; u64::from_le_bytes(len) as usize]; 
//                  buf_reader
//                        .read_exact(&mut str_data)
//                        .expect("segfault: str data after length");
//                    chunk.append(&mut str_data);
//                    &chunk
//                },
//                _ => &chunk
//            };
//        }
//        return BytecodeFile {
//            bytecode: vec![],
//            consts: vec![],
//            version: match std::str::from_utf8(version) {
//                Ok(v) => String::from(v),
//                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//            }
//        };
//    }
//}
//

fn read_fnname (buf_reader: &mut BufReader<File>) -> String {
    let mut fnnname = String::new();
    unsafe {
        //make sure all names are utf8
        let _ = buf_reader.read_until(b'(', fnnname.as_mut_vec());
    }
    fnnname
}

impl BytecodeFile {
    pub fn new(file_path: String) -> BytecodeFile {

        let bincode_config = bincode::config::standard().with_fixed_int_encoding();
        let mut buf_reader = File::open(&file_path)
            .map(BufReader::new)
            .expect(stringify!("No file found ", file_path));

        let mut vers = [0; 3];
        buf_reader
            .read_exact(&mut vers)
            .unwrap();
        let mut bytecode = Vec::new();

        let mut len = [0;1];
        buf_reader
            .read(&mut len)
            .unwrap();
        
        let mut consts = Vec::new();    
        for _ in 0..*len.first().unwrap() {
            let mut chunk: Vec<u8> = vec![];
            //This is where the bincode discriminator. It will be 4 bytes 
            let mut discrim = [0u8; 4]; 
            //We read it here^
            buf_reader
                .read_exact(&mut discrim)
                .expect("Segfault: misaligned chunk of const data");
            //We add the discrim to the slice
            chunk.extend_from_slice(&discrim);
            match u32::from_le_bytes(discrim) {
                0 => {
                    let mut num = [0u8; 8];
                    buf_reader.read_exact(&mut num).expect("Segfault: could not find number value after discrim");
                    chunk.extend_from_slice(&num);
                    &chunk
                }
                1 => { 
                    let mut bool = [0u8; 1];
                    buf_reader.read_exact(&mut bool).expect("Segfault: could not find boolean value after discrim");
                    chunk.push(*bool.first().unwrap());
                    &chunk
                }
                2 => {
                  let mut len = [0u8; 8];
                  buf_reader.read_exact(&mut len).expect("Segfault: missing chunk of data for string length");
                  chunk.extend_from_slice(&len);
                  let mut str_data = vec![0; u64::from_le_bytes(len) as usize]; 
                  buf_reader
                        .read_exact(&mut str_data)
                        .expect("segfault: str data after length");
                    chunk.append(&mut str_data);
                    &chunk
                },
                _ => &chunk
            };
            let value = bincode::decode_from_slice::<Value, _>(&chunk, bincode_config);
            let value = value.unwrap().0;
            consts.push(value);
        }
        let mut functions = Vec::new();
        let mut fnlen = [0;1];
        buf_reader
            .read(&mut fnlen)
            .unwrap();
        for _ in 0..*fnlen.first().unwrap() {
            let name = read_fnname(&mut buf_reader);
            //skip (
            buf_reader
                .seek_relative(1)
                .expect("Byte ( to be skipped");
            let mut fnarity = [0;1];
            buf_reader
                .read(&mut fnarity)
                .expect("expected a number after (");
            //skip )
            buf_reader
                .seek_relative(1)
                .expect("Byte ) to be skipped");
            let mut fn_bytecode = Vec::new();
            buf_reader
                .read_until(b'\0', &mut fn_bytecode)
                .expect("expected bytecode");
            functions.push(
                Function::new(name, *fnarity.first().unwrap(), fn_bytecode)
            );
        }
        functions.sort_by(|a, b| a.name.cmp(&b.name));
                
        buf_reader
            .read_to_end(&mut bytecode)
            .unwrap();
        //consts.iter().for_each(|v| println!("{:?}", v) );
        BytecodeFile {
            bytecode,
            consts,
            functions: HashMap::new(),
            version: match std::str::from_utf8(&vers) {
                Ok(v) => String::from(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            }
        }
    }

    pub fn get_byte(&self, index: usize) -> Option<u8> {
        self.bytecode
            .get(index)
            .map(|o| *o)
    }

    pub fn get_const(&self, idx: usize) -> Option<&Value> {
        self.consts.get(idx)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(5, 5);
    }
}
