use std::fmt;

use nom;
#[derive(Debug)]
pub struct ExecError {
    details: String,
}

impl ExecError {
    pub fn new(details: &str) -> Self {
        return ExecError {
            details: details.to_string(),
        };
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.details)
    }
}

impl std::error::Error for ExecError {}

#[derive(Debug, Clone, PartialEq, Eq)]
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
