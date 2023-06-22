extern crate core;

pub mod structures;
pub mod vm;

use clap::Parser;
use structures::compile_unit::CompileUnit;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

}


pub fn main() {
    let args = Args::parse();
    let path = args.path;
    
    let compile_unit = CompileUnit::new(path);
    

}
