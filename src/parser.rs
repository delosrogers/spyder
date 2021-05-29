// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.

use crate::types::{Instruction, Instruction::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending},
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
    // you cannot label a goto or goto if equal
    LabeledIns(LabeledIns<'a>),
    Goto(Label<'a>),
    GotoEqual(Label<'a>),
    // Call(Label<'a>),
}

pub type Label<'a> = &'a str;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledIns<'a> {
    pub label: Label<'a>,
    pub ins: Instruction,
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
        alt((labled_ins, goto, push, plain_statement)),
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
            Statement::LabeledIns(LabeledIns {
                label: res.1,
                ins: match res.3 {
                    Statement::Ins(instruction) => instruction,
                    _ => panic!("statement following label should be a plain ins"),
                },
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

// fn call(input: &str) -> Res<&str, Statement> {
//     context(
//         "call"
//         tuple( )
//     )
// }

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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_code() {
        let res = code("goto END\r\n!![END] rePush");
        assert_eq!(
            res,
            Ok((
                "",
                Code {
                    lines: vec![
                        Statement::Goto("END"),
                        Statement::LabeledIns(LabeledIns {
                            label: "END",
                            ins: RePush
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
                Statement::LabeledIns(LabeledIns {
                    label: "END",
                    ins: Mul,
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
                Statement::LabeledIns(LabeledIns {
                    label: "END",
                    ins: Mul,
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
}
