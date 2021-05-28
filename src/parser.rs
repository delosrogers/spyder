use crate::types::{Instruction, Instruction::*, ParseError};

pub fn parse_instruction(str_instruction: &str) -> Result<Instruction, ParseError> {
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
