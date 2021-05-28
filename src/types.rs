use core::fmt;

use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
    #[structopt(short, long)]
    pub debug: bool,
}

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

pub struct Interpreter {
    pub stack: Vec<i64>,
    pub vars: Vec<i64>,
    pub curr_instruction_idx: usize,
}

impl Interpreter {
    pub fn pop(&mut self) -> Result<i64, ParseError> {
        match self.stack.pop() {
            Some(num) => Ok(num),
            None => Err(self.empty_stack_err()),
        }
    }
    pub fn empty_stack_err(&self) -> ParseError {
        let mut error_msg = "stack empty at line: ".to_owned();
        error_msg.push_str(self.curr_instruction_idx.to_string().as_str());
        ParseError::new(error_msg.as_str())
    }
    pub fn get_val(&self, source: &i64) -> Result<i64, ParseError> {
        Ok(self.vars[*source as usize])
    }
    pub fn set_val(&mut self, dest: i64, val: i64) {
        if self.vars.len() < dest as usize {
            self.vars.resize((dest + 10) as usize, 0)
        }
        self.vars[dest as usize] = val
    }
    pub fn last(&self) -> Result<i64, ParseError> {
        match self.stack.last() {
            Some(num) => Ok(*num),
            None => Err(self.empty_stack_err()),
        }
    }
}

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
