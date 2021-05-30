// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.

use crate::types::{Instruction, Instruction::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending, not_line_ending},
    combinator::opt,
    error::{context, VerboseError},
    multi::many1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Code<'a> {
    pub lines: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    Ins(Instruction),
    VarExpr(VariableExpr<'a>),
    // you cannot label a goto or goto if equal
    LabeledStatement(LabeledStatement<'a>),
    Goto(Label<'a>),
    GotoEqual(Label<'a>),
    Call(Label<'a>),
    Comment(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableExpr<'a> {
    Assignment(Assignment<'a>),
    Access(VarAccess<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment<'a> {
    pub name: &'a str,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarAccess<'a> {
    pub load_or_store: LoadOrStore,
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadOrStore {
    Load,
    Store,
}

impl LoadOrStore {
    pub fn get_value(&self) -> Instruction {
        match self {
            Self::Load => Instruction::Load,
            Self::Store => Instruction::Store,
        }
    }
}

pub type Label<'a> = &'a str;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledStatement<'a> {
    pub label: Label<'a>,
    pub statement: Box<Statement<'a>>,
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn code(input: &str) -> Res<&str, Code> {
    context(
        "code",
        tuple((many1(tuple((statement, line_ending))), statement)),
    )(input)
    .map(|(next_input, res)| {
        let mut lines: Vec<Statement> = Vec::new();
        res.0.iter().for_each(|(stmt, _)| lines.push(stmt.clone()));
        lines.push(res.1);
        (next_input, Code { lines })
    })
}

fn statement(input: &str) -> Res<&str, Statement> {
    context(
        "instruction",
        alt((
            labled_ins,
            goto,
            push,
            call,
            comment,
            variable_expression,
            plain_statement,
        )),
    )(input)
}

fn labled_ins(input: &str) -> Res<&str, Statement> {
    context(
        "labled ins",
        tuple((tag("!!["), alphanumeric1, tag("] "), statement)),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Statement::LabeledStatement(LabeledStatement {
                label: res.1,
                statement: Box::new(res.3),
            }),
        )
    })
}

fn goto(input: &str) -> Res<&str, Statement> {
    context(
        "goto or goto if equal",
        alt((
            separated_pair(tag("goto"), tag(" "), alphanumeric1),
            separated_pair(tag("gotoEqual"), tag(" "), alphanumeric1),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            match res.0 {
                "goto" => Statement::Goto(res.1),
                "gotoEqual" => Statement::GotoEqual(res.1),
                _ => panic!("goto parser tried to parse non goto or gotoifequal"),
            },
        )
    })
}

fn call(input: &str) -> Res<&str, Statement> {
    context("call", separated_pair(tag("call"), tag(" "), alphanumeric1))(input)
        .map(|(next_input, res)| (next_input, Statement::Call(res.1)))
}

fn push(input: &str) -> Res<&str, Statement> {
    context("push", separated_pair(tag("push"), tag(" "), number))(input).map(
        |(next_input, res)| {
            let num = res.1;
            (next_input, Statement::Ins(Push(num)))
        },
    )
}

// turns positive or negative decimal numbers to i64
fn number(input: &str) -> Res<&str, i64> {
    context("is plus-minus digit", tuple((opt(tag("-")), digit1)))(input).map(
        |(next_input, res)| {
            let multiple = if res.0.is_some() { -1 } else { 1 };
            let num: i64 = res.1.parse::<i64>().expect("not a number");
            (next_input, num * multiple)
        },
    )
}

fn variable_expression(input: &str) -> Res<&str, Statement> {
    context("expression with variables", alt((assignment, load_store)))(input)
        .map(|(next_input, res)| (next_input, Statement::VarExpr(res)))
}

fn assignment(input: &str) -> Res<&str, VariableExpr> {
    context(
        "assignment",
        tuple((
            tag("var "),
            separated_pair(alphanumeric1, tag(" = "), number),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            VariableExpr::Assignment(Assignment {
                name: res.1 .0,
                value: res.1 .1,
            }),
        )
    })
}

fn load_store(input: &str) -> Res<&str, VariableExpr> {
    context(
        "load store",
        separated_pair(
            alt((tag("load"), tag("store"))),
            opt(tag(" ")),
            opt(alphanumeric1),
        ),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            VariableExpr::Access(VarAccess {
                load_or_store: match res.0 {
                    "load" => LoadOrStore::Load,
                    "store" => LoadOrStore::Store,
                    _ => panic!("load_store parser got something other than load store"),
                },
                name: res.1,
            }),
        )
    })
}

/// Parses parameter less instructions into Instruction
/// i.e. not Push or Goto
fn plain_statement(input: &str) -> Res<&str, Statement> {
    context(
        "plain instruction",
        alt((
            tag("load"),
            tag("store"),
            tag("pop"),
            tag("rePush"),
            tag("noOp"),
            tag("return"),
            tag("add"),
            tag("sub"),
            tag("mul"),
            tag("div"),
        )),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Statement::Ins(match res {
                "load" => Load,
                "store" => Store,
                "pop" => Pop,
                "rePush" => RePush,
                "return" => Goto,
                "add" => Add,
                "sub" => Sub,
                "mul" => Mul,
                "div" => Div,
                "noOp" => NoOp,
                _ => panic!("plain instruction tried to parse an non plain instruciton"),
            }),
        )
    })
}

fn comment(input: &str) -> Res<&str, Statement> {
    context("comment", tuple((tag("//"), not_line_ending)))(input)
        .map(|(next_input, res)| (next_input, Statement::Comment(res.1)))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_code() {
        let res = code("// comment here\r\ngoto END\r\n!![END] rePush");
        assert_eq!(
            res,
            Ok((
                "",
                Code {
                    lines: vec![
                        Statement::Comment(" comment here"),
                        Statement::Goto("END"),
                        Statement::LabeledStatement(LabeledStatement {
                            label: "END",
                            statement: Box::new(Statement::Ins(RePush)),
                        })
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_statement() {
        let res = statement("!![END] mul");
        assert_eq!(
            res,
            Ok((
                "",
                Statement::LabeledStatement(LabeledStatement {
                    label: "END",
                    statement: Box::new(Statement::Ins(Mul)),
                })
            ))
        )
    }

    #[test]
    fn test_labled_ins() {
        let res = labled_ins("!![END] mul");
        assert_eq!(
            res,
            Ok((
                "",
                Statement::LabeledStatement(LabeledStatement {
                    label: "END",
                    statement: Box::new(Statement::Ins(Mul))
                })
            ))
        )
    }

    #[test]
    fn test_goto() {
        let mut res = goto("goto END");
        assert_eq!(res, Ok(("", Statement::Goto("END"))));
        res = goto("gotoEqual TopOfLoop");
        assert_eq!(res, Ok(("", Statement::GotoEqual("TopOfLoop"))));
    }

    #[test]
    fn test_push() {
        let res = push("push -50");
        assert_eq!(res, Ok(("", Statement::Ins(Push(-50)))))
    }

    #[test]
    fn test_call() {
        let res = call("call PerformCalc2");
        assert_eq!(res, Ok(("", Statement::Call("PerformCalc2"))));
    }
    #[test]
    fn test_var_assignment() {
        let res = assignment("var foo = 5");
        assert_eq!(
            res,
            Ok((
                "",
                VariableExpr::Assignment(Assignment {
                    name: "foo",
                    value: 5,
                })
            ))
        );
    }

    #[test]
    fn test_load() {
        let mut res = load_store("load foo");
        assert_eq!(
            res,
            Ok((
                "",
                VariableExpr::Access(VarAccess {
                    load_or_store: LoadOrStore::Load,
                    name: Some("foo"),
                })
            ))
        );
        res = load_store("load");
        assert_eq!(
            res,
            Ok((
                "",
                VariableExpr::Access(VarAccess {
                    load_or_store: LoadOrStore::Load,
                    name: None,
                })
            ))
        );
    }

    #[test]
    fn test_store() {
        let mut res = load_store("store foo");
        assert_eq!(
            res,
            Ok((
                "",
                VariableExpr::Access(VarAccess {
                    load_or_store: LoadOrStore::Store,
                    name: Some("foo"),
                })
            ))
        );
        res = load_store("store");
        assert_eq!(
            res,
            Ok((
                "",
                VariableExpr::Access(VarAccess {
                    load_or_store: LoadOrStore::Store,
                    name: None,
                })
            ))
        );
    }
}
