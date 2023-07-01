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
    // let args = Args::parse();
    // let path = args.path;
    let path = "";
    let compile_unit = BytecodeFile::new(path.to_string());

    println!("Version {}", &compile_unit.version)
    
    

}
