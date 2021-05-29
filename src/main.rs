pub mod codegen;
pub mod interpreter;
pub mod parser;
pub mod types;
use crate::interpreter::Interpreter;
use std::fs;
use std::str;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
    #[structopt(short, long)]
    pub debug: bool,
}

fn main() {
    let args = Cli::from_args();
    let file = fs::read_to_string(&args.path).expect("problem reading file");
    let contents: Vec<&str> = file.lines().collect();
    let parsed = parser::code(file.as_str()).expect("parser error").1;
    let lowered = codegen::lowerIR(&parsed).expect("parser error");
    println!("{:?}", lowered);
    let mut vm = Interpreter::new();
    let result = vm.run(lowered, args.debug);
    println!("{:?}", result);
}
