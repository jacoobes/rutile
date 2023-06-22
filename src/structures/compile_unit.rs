use std::fs::File;
use std::io::Read;
//numsc compilation unit is a file
//anything that is included is part of the compilation unit.
//usually the compilation unit has an entry point (main)

pub struct CompileUnit {
    file_path: String,
    file: File,
    version: String
}


impl CompileUnit {
    pub fn new(file_path: String) -> CompileUnit {
        let mut f = File::open(&file_path).unwrap();
        let mut buf = [0; 3];
        f.read_exact(&mut buf).unwrap();
        CompileUnit {
            file_path,
            file: f,
            version: match std::str::from_utf8(&buf) {
                Ok(v) => String::from(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            }
        }
    }
}

impl Drop for CompileUnit {
    fn drop(&mut self) {
        // Close the file when the struct is dropped
        core::mem::drop(&self.file)
    }
}
