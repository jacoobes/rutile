use std::collections::HashMap;
use std::io::BufRead;
use std::io::Read;
use std::io::BufReader;
use std::io::Seek;

use super::function::Function;
use super::value::Value;

//anything that is included is part of the compilation unit.
//usually the compilation unit has an entry point (main)

pub struct BytecodeFile {
    bytecode: Vec<u8>,
    functions: HashMap<String, Function>,
    consts: Vec<Value>,
    pub version: String,
}

fn read_one<R: Read + Seek>(reader: &mut BufReader<R>)-> Result<u8, std::io::Error>  {
    let mut buffer = [0u8; 1]; // Create a buffer to store the read byte

    // Seek to the current position to ensure you are reading from the current position
    reader.seek(std::io::SeekFrom::Current(0))?;

    // Read one byte into the buffer
    let bytes_read = reader.read(&mut buffer)?;

    if bytes_read == 1 {
        Ok(buffer[0]) // Successfully read one byte
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Failed to read one byte",
        ))
    }
}
fn read_fnname<R: Read>(buf_reader: &mut BufReader<R>) -> String {
    let mut fnnname = String::new();
    unsafe {
        //make sure all names are utf8
        buf_reader
            .read_until(b'(', fnnname.as_mut_vec())
            .expect("Unable to read bytes while reading a fnnname");
    }
    fnnname.pop();
    fnnname
}

impl BytecodeFile {
    pub fn new<R>(readable: R) -> BytecodeFile where R: Read + Seek {

        let bincode_config = bincode::config::standard().with_fixed_int_encoding();
        let mut buf_reader = BufReader::new(readable);

        let mut vers = [0; 5];
        buf_reader
            .read_exact(&mut vers)
            .unwrap();
        let mut bytecode = Vec::new();

        let len = read_one(&mut buf_reader);
        
        let mut consts = Vec::new();    
        for _ in 0..len.unwrap() {
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
        let mut functions = HashMap::new();
        let fnlen = read_one(&mut buf_reader).expect("function table count not read");
        for _ in 0..fnlen {
            let name = read_fnname(&mut buf_reader);
            let fnarity = read_one(&mut buf_reader)
                .expect("expected a number after (");
            //skip )
            buf_reader
                .seek_relative(1)
                .expect("Byte ) to be skipped");
            let mut fn_bytecode = Vec::new();
            buf_reader
                .read_until(b'\0', &mut fn_bytecode)
                .expect("expected bytecode");
            //clone
            functions.insert(
                name.clone(),
                Function::new(name, fnarity, fn_bytecode));
        }
                
        buf_reader
            .read_to_end(&mut bytecode)
            .unwrap();
        consts.iter().for_each(|v| println!("{:?}", v) );
        BytecodeFile {
            bytecode,
            consts,
            functions,
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
    use std::io::Cursor;
    use super::*;
    #[test]
    fn test_header() {
        let bytecode: Vec<u8> = vec![
           //HEADER = KGC2
           127, 75, 71, 67, 50, 0, 0
        ];
        let c = Cursor::new(bytecode);
        let bcf = BytecodeFile::new(c);
        assert_eq!(bcf.version, String::from("\u{7f}KGC2"));
        assert_eq!(bcf.functions.len(),0);
        assert_eq!(bcf.consts.len() ,0);
    }

     #[test]
    fn test_consts() {
        let bytecode: Vec<u8> = vec![
           //HEADER = KGC2
           127, 75, 71, 67, 50,
           //2 consts in the table
           2, 
           //enum discrim of 1 (this will be a Value::Bool(true)
           1,0,0,0, 1,

           //enum discrim of 1 (this will be a Value::Bool(false)
           1,0,0,0, 0,

           //enum discrim of 1 (this will be a Value::Number(f64)
           //0,0,0,0, 42, 0xca, 0, 0, 0, 0, 0, 0, 
           //0 functions in the bytecode
           0
        ];
        let c = Cursor::new(bytecode);
        let bcf = BytecodeFile::new(c);
        assert_eq!(bcf.consts[0], Value::Boolean(true));
        assert_eq!(bcf.consts[1], Value::Boolean(false));
        //assert_eq!(bcf.consts[2], Value::Number(101f64));
    }
    #[test]
    fn test_functions() {
        let bytecode: Vec<u8> = vec![
           //HEADER = KGC2
           127, 75, 71, 67, 50,
           //2 consts in the table
           2, 
           //enum discrim of 1 (this will be a Value::Bool(true)
           1,0,0,0, 1,

           //enum discrim of 1 (this will be a Value::Bool(false)
           1,0,0,0, 0,
           //1 functions in the bytecode
           1,
           b'd', b'e', b'e', b'z',
           b'(', 2, b')'
        ];
        let c = Cursor::new(bytecode);
        let bcf = BytecodeFile::new(c);
        assert_eq!(bcf.consts[0], Value::Boolean(true));
        assert_eq!(bcf.consts[1], Value::Boolean(false));

        assert_eq!(bcf.functions.len(), 1);
        assert!(bcf.functions.get("deez").is_some());
        
        let deezfn = bcf.functions.get("deez").unwrap();
        assert_eq!(deezfn.arity, 2)
    }


}
