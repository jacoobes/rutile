use std::fs::File;
use std::io::Read;
use std::io::BufReader;

use super::value::Value;
//numsc compilation unit is a file
//anything that is included is part of the compilation unit.
//usually the compilation unit has an entry point (main)

pub struct BytecodeFile {
    bytecode: Vec<u8>,
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

        buf_reader
            .read_to_end(&mut bytecode)
            .unwrap();
        consts.iter().for_each(|v| println!("{:?}", v) );
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
