extern crate core;

pub mod structures;
pub mod vm;

use std::fs::File;

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
    let f = File::open(args.path.to_string())
                .expect("Unable to open file");
    let compile_unit = BytecodeFile::new(f);
    println!("Version {}", &compile_unit.version);
    //let result = vm::frame_reader::interpret_unit(compile_unit, locals);
}
