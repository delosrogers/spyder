use std::fmt;
#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub fn new(details: &str) -> Self {
        return ParseError {
            details: details.to_string(),
        };
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.details)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub enum Instruction {
    Push(i64),
    // pops source off of stack then pushes val onto stack
    Load,
    // pops destination of stack then pops into dest
    Store,
    Pop,
    Goto,
    // pops sentinal off stack
    GotoIfEqual,
    // Pushes a second copy of what is at the top of the stack
    RePush,
    Add,
    Sub,
    Mul,
    Div,
}
