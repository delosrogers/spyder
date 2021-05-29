use crate::parser;
use crate::types::{ExecError, Instruction, Instruction::*};

pub struct Interpreter {
    pub stack: Vec<i64>,
    pub vars: Vec<i64>,
    pub curr_instruction_idx: usize,
}
impl Interpreter {
    pub fn pop(&mut self) -> Result<i64, ExecError> {
        match self.stack.pop() {
            Some(num) => Ok(num),
            None => Err(self.empty_stack_err()),
        }
    }
    pub fn empty_stack_err(&self) -> ExecError {
        let mut error_msg = "stack empty at line: ".to_owned();
        error_msg.push_str(self.curr_instruction_idx.to_string().as_str());
        ExecError::new(error_msg.as_str())
    }
    pub fn get_val(&self, source: &i64) -> Result<i64, ExecError> {
        Ok(self.vars[*source as usize])
    }
    pub fn set_val(&mut self, dest: i64, val: i64) {
        if self.vars.len() < dest as usize {
            self.vars.resize((dest + 10) as usize, 0)
        }
        self.vars[dest as usize] = val
    }
    pub fn last(&self) -> Result<i64, ExecError> {
        match self.stack.last() {
            Some(num) => Ok(*num),
            None => Err(self.empty_stack_err()),
        }
    }
    pub fn run(&mut self, instructions: Vec<Instruction>, debug: bool) -> Result<i64, ExecError> {
        while self.curr_instruction_idx < instructions.len() {
            let instruction = &instructions[self.curr_instruction_idx];
            if debug {
                println!("stack {:?}", self.stack);
                println!("{}: {:?}", self.curr_instruction_idx, instruction);
            }
            match instruction {
                Push(val) => self.stack.push(*val),
                Load => {
                    let source = self.pop()?;
                    // TODO improve error handling
                    let val = self.get_val(&source).unwrap();
                    self.stack.push(val);
                }
                Store => {
                    let dest = self.pop()?;
                    let val = self.pop()?;
                    self.set_val(dest, val);
                }
                Pop => {
                    self.pop()?;
                }
                Goto => {
                    let new_line = self.pop()?;
                    // need to subtract 1 because i is incremented every time
                    self.curr_instruction_idx = new_line as usize - 1;
                }
                GotoIfEqual => {
                    let new_line = self.pop()?;
                    let sentinal = self.pop()?;
                    if sentinal == 0 {
                        self.curr_instruction_idx = new_line as usize - 1;
                    }
                }
                RePush => self.stack.push(self.last()?),
                Add => {
                    let arg1 = self.pop()?;
                    let arg2 = self.pop()?;
                    self.stack.push(arg1 + arg2);
                }
                Sub => {
                    let arg1 = self.pop()?;
                    let arg2 = self.pop()?;
                    self.stack.push(arg1 - arg2);
                }
                Mul => {
                    let arg1 = self.pop()?;
                    let arg2 = self.pop()?;
                    self.stack.push(arg1 * arg2);
                }
                Div => {
                    let arg1 = self.pop()?;
                    let arg2 = self.pop()?;
                    self.stack.push(arg1 / arg2);
                }
                NoOp => (),
            }
            self.curr_instruction_idx += 1;
        }
        return self.pop();
    }
    pub fn new() -> Self {
        return Self {
            stack: Vec::new(),
            vars: Vec::new(),
            curr_instruction_idx: 0,
        };
    }
}
