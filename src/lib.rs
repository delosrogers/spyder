// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.

pub mod codegen;
pub mod interpreter;
pub mod parser;
pub mod types;
use crate::interpreter::Interpreter;
use crate::types::ExecError;
use std::{fs, path::PathBuf};

pub fn run_file(path: &PathBuf, debug: bool) -> Result<i64, ExecError> {
    let file = fs::read_to_string(path).expect("problem reading file");
    let parsed = parser::code(file.as_str()).expect("parser error").1;
    let lowered = codegen::lower_IR(&parsed)?;
    println!("{:?}", lowered);
    let mut vm = Interpreter::new();
    vm.run(lowered, debug)
}
