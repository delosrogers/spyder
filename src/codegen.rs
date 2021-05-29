// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.
use std::collections::HashMap;

use crate::parser::{Assignment, Code, Statement, VarAccess, VariableExpr};
use crate::types::{ExecError, Instruction};

pub struct CodeGen<'a> {
    labels: HashMap<&'a str, usize>,
    // stores the label and index of the push that should
    // push the index of what that label refers to
    label_refs: Vec<(&'a str, usize)>,
    labels_resolved: Vec<Instruction>,
    curr_base_idx: usize,
    variable_names: HashMap<&'a str, usize>,
    curr_variable_allocation_idx: usize,
}

impl<'a> CodeGen<'a> {
    #[allow(non_snake_case)]
    pub fn lower_IR(&mut self, input: &'a Code) -> Result<Vec<Instruction>, ExecError> {
        for stmt in input.lines.iter() {
            self.curr_base_idx = self.labels_resolved.len();
            match stmt {
                Statement::LabeledIns(labled_ins) => {
                    self.labels.insert(labled_ins.label, self.curr_base_idx);
                    self.labels_resolved.push(labled_ins.ins.clone());
                }
                Statement::Goto(goto) => {
                    self.label_refs.push((*goto, self.curr_base_idx));
                    self.labels_resolved.push(Instruction::Push(0));
                    self.labels_resolved.push(Instruction::Goto);
                }
                Statement::GotoEqual(goto_if_equal) => {
                    self.label_refs.push((*goto_if_equal, self.curr_base_idx));
                    self.labels_resolved.push(Instruction::Push(0));
                    self.labels_resolved.push(Instruction::GotoEqual);
                }
                Statement::Call(label) => {
                    // push return addr
                    self.labels_resolved.push(Instruction::ClearStack);
                    self.labels_resolved
                        .push(Instruction::Push((self.curr_base_idx + 4) as i64));
                    self.label_refs.push((*label, self.curr_base_idx + 2));
                    self.labels_resolved.push(Instruction::Push(0));
                    self.labels_resolved.push(Instruction::Goto);
                }
                Statement::VarExpr(var_expr) => self.lower_var_expr(var_expr)?,
                Statement::Ins(instruction) => self.labels_resolved.push(instruction.clone()),
                Statement::Comment(_) => (),
            }
        }
        // fill in go to destinations using labels map
        for (label, position) in self.label_refs.iter() {
            let jump_dest = match self.labels.get(*label) {
                Some(&idx) => idx,
                None => {
                    let mut error_string = "Could not find matching label to: ".to_owned();
                    error_string.push_str(*label);
                    return Err(ExecError::new(error_string.as_str()));
                }
            };
            self.labels_resolved[*position] = Instruction::Push(jump_dest as i64)
        }
        Ok(self.labels_resolved.clone())
    }

    fn lower_var_expr(&mut self, var_expr: &'a VariableExpr) -> Result<(), ExecError> {
        match var_expr {
            VariableExpr::Assignment(assignment_expr) => self.lower_assignment(assignment_expr),
            VariableExpr::Access(access_expr) => self.lower_access(access_expr)?,
        }
        Ok(())
    }

    fn lower_access(&mut self, access: &'a VarAccess) -> Result<(), ExecError> {
        match access.name {
            None => self.labels_resolved.push(access.load_or_store.get_value()),
            Some(name) => {
                let addr = match self.variable_names.get(name) {
                    Some(addr) => *addr,
                    None => {
                        let mut err_string = "uninitialized variable: ".to_string();
                        err_string.push_str(name);
                        return Err(ExecError::new(err_string.as_str()));
                    }
                };
                self.labels_resolved.push(Instruction::Push(addr as i64));
                self.labels_resolved.push(access.load_or_store.get_value());
            }
        }
        Ok(())
    }

    fn lower_assignment(&mut self, assignment: &'a Assignment) {
        match self.variable_names.get(assignment.name) {
            Some(addr) => {
                self.labels_resolved
                    .push(Instruction::Push(assignment.value));
                self.labels_resolved.push(Instruction::Push(*addr as i64));
                self.labels_resolved.push(Instruction::Store);
            }
            None => {
                self.curr_variable_allocation_idx += 1;
                self.variable_names
                    .insert(assignment.name, self.curr_variable_allocation_idx);
                self.labels_resolved
                    .push(Instruction::Push(assignment.value));
                self.labels_resolved
                    .push(Instruction::Push(self.curr_variable_allocation_idx as i64));
                self.labels_resolved.push(Instruction::Store);
            }
        }
    }
    pub fn new() -> Self {
        Self {
            label_refs: vec![],
            labels: HashMap::new(),
            labels_resolved: vec![],
            variable_names: HashMap::new(),
            curr_variable_allocation_idx: 0,
            curr_base_idx: 0,
        }
    }
}
