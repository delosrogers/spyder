// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.
use std::collections::HashMap;

use crate::parser::{Code, Statement};
use crate::types::{ExecError, Instruction};

#[allow(non_snake_case)]
pub fn lower_IR(input: &Code) -> Result<Vec<Instruction>, ExecError> {
    let mut labels: HashMap<&str, usize> = HashMap::new();

    // stores the label and index of the push that should
    // push the index of what that label refers to
    let mut label_refs: Vec<(&str, usize)> = Vec::new();
    let mut labels_resolved: Vec<Instruction> = Vec::new();
    for stmt in input.lines.iter() {
        let i = labels_resolved.len();
        match stmt {
            Statement::LabeledIns(labled_ins) => {
                labels.insert(labled_ins.label, i);
                labels_resolved.push(labled_ins.ins.clone());
            }
            Statement::Goto(goto) => {
                label_refs.push((*goto, i));
                labels_resolved.push(Instruction::Push(0));
                labels_resolved.push(Instruction::Goto);
            }
            Statement::GotoEqual(goto_if_equal) => {
                label_refs.push((*goto_if_equal, i));
                labels_resolved.push(Instruction::Push(0));
                labels_resolved.push(Instruction::GotoEqual);
            }
            Statement::Ins(instruction) => labels_resolved.push(instruction.clone()),
        }
    }
    // fill in go to destinations using labels map
    for (label, position) in label_refs.iter() {
        let jump_dest = match labels.get(*label) {
            Some(&idx) => idx,
            None => {
                let mut error_string = "Could not find matching label to: ".to_owned();
                error_string.push_str(*label);
                return Err(ExecError::new(error_string.as_str()));
            }
        };
        labels_resolved[*position] = Instruction::Push(jump_dest as i64)
    }
    Ok(labels_resolved)
}
