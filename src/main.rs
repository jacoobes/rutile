extern crate core;

pub mod structures;
pub mod vm;

use clap::Parser;
use structures::bytecode_file::BytecodeFile;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

}


pub fn main() {
    let args = Args::parse();
    let compile_unit = BytecodeFile::new(args.path.to_string());

    println!("Version {}", &compile_unit.version);
    
    let result = vm::frame_reader::interpret_unit(compile_unit);
    
    
}
