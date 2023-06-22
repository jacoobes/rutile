use std::io::BufReader;
use std::fs::File;
//numsc compilation unit is a file
//anything that is included is part of the compilation unit.
//usually the compilation unit has an entry point (main)
//
struct CompileUnit {
    file_path: String,
    buf_reader: BufReader<File>
}


impl CompileUnit {
    pub fn new(file_path: String) -> Result<CompileUnit, std::io::Error> {
        let f = File::open(file_path)?;
        Ok(CompileUnit { file_path, buf_reader: BufReader::new(f) })
    }
    
}
