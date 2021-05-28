pub mod types;
use crate::types::{Cli, Instruction, Instruction::*, Interpreter, ParseError};
use std::fs;
use std::str;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let file = fs::read_to_string(&args.path).expect("problem reading file");
    let contents: Vec<&str> = file.lines().collect();
    let contents2 = contents.clone();
    let result = interpret(contents, args.debug);
    let result2 = interpret(contents2, args.debug);
    println!("{:?}", result);
    println!("{:?}", result2);
}

fn interpret(code: Vec<&str>, debug: bool) -> Result<i64, ParseError> {
    // Stands for first language
    let mut vm = Interpreter {
        stack: Vec::new(),
        vars: Vec::new(),
        curr_instruction_idx: 0,
    };
    let instructions: Vec<Instruction> = code
        .into_iter()
        .map(|instruction_str| parse_instruction(instruction_str).expect("problem parsing"))
        .collect();
    while vm.curr_instruction_idx < instructions.len() {
        let instruction = &instructions[vm.curr_instruction_idx];
        if debug {
            println!("stack {:?}", vm.stack);
            println!("{}: {:?}", vm.curr_instruction_idx, instruction);
        }
        match instruction {
            Push(val) => vm.stack.push(*val),
            Load => {
                let source = vm.pop()?;
                // TODO improve error handling
                let val = vm.get_val(&source).unwrap();
                vm.stack.push(val);
            }
            Store => {
                let dest = vm.pop()?;
                let val = vm.pop()?;
                vm.set_val(dest, val);
            }
            Pop => {
                vm.pop()?;
            }
            Goto => {
                let new_line = vm.pop()?;
                // need to subtract 1 because i is incremented every time
                vm.curr_instruction_idx = new_line as usize - 1;
            }
            GotoIfEqual => {
                let new_line = vm.pop()?;
                let sentinal = vm.pop()?;
                if sentinal == 0 {
                    vm.curr_instruction_idx = new_line as usize - 1;
                }
            }
            RePush => vm.stack.push(vm.last()?),
            Add => {
                let arg1 = vm.pop()?;
                let arg2 = vm.pop()?;
                vm.stack.push(arg1 + arg2);
            }
            Sub => {
                let arg1 = vm.pop()?;
                let arg2 = vm.pop()?;
                vm.stack.push(arg1 - arg2);
            }
            Mul => {
                let arg1 = vm.pop()?;
                let arg2 = vm.pop()?;
                vm.stack.push(arg1 * arg2);
            }
            Div => {
                let arg1 = vm.pop()?;
                let arg2 = vm.pop()?;
                vm.stack.push(arg1 / arg2);
            }
        }
        vm.curr_instruction_idx += 1;
    }
    return vm.pop();
}

fn parse_instruction(str_instruction: &str) -> Result<Instruction, ParseError> {
    let mut tokens = str_instruction.split_whitespace();
    let first_word = match tokens.next() {
        Some(string) => string,
        None => return Err(ParseError::new("parse_instruction: empty line")),
    };
    let instruction = match first_word {
        "Push" => match tokens.next() {
            Some(num) => Instruction::Push(num.parse::<i64>().expect("not a number")),
            None => return Err(ParseError::new("parse_instruction: no number to push")),
        },
        "Load" => Load,
        "Store" => Store,
        "Pop" => Pop,
        "Goto" => Goto,
        "GotoIfEqual" => GotoIfEqual,
        "RePush" => RePush,
        "Add" => Add,
        "Sub" => Sub,
        "Mul" => Mul,
        "Div" => Div,
        _ => return Err(ParseError::new(str_instruction)),
    };
    Ok(instruction)
}
