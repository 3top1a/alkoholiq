// parser/mod.rs
use pest::Parser;
use pest_derive::Parser;
use pest::iterators::{Pair, Pairs};
use std::str::FromStr;

use crate::lir::lir::{
    BinaryOp, Instruction, Location, Value,
    Instruction::*,
};
use crate::lir::instructions::Instructions;

#[derive(Parser)]
#[grammar = "lir/grammar.pest"]
struct LirParser;

#[derive(Debug)]
pub enum ParseError {
    PestError(pest::error::Error<Rule>),
    InvalidInstruction(String),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(error)
    }
}

pub fn parse(input: &str) -> Result<Instructions, ParseError> {
    let pairs = LirParser::parse(Rule::program, input)?;
    let instructions = parse_pairs(pairs)?;
    Ok(instructions.into())
}

fn parse_pairs(pairs: Pairs<Rule>) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::instruction => {
                instructions.push(parse_instruction(pair)?);
            },
            Rule::EOI | Rule::program => {
                // Handle nested instructions recursively
                instructions.extend(parse_pairs(pair.into_inner())?);
            },
            _ => {
                // Debug log or ignore other rules
                println!("Skipping rule: {:?}", pair.as_rule());
            }
        }
    }

    Ok(instructions)
}

fn parse_instruction(pair: Pair<Rule>) -> Result<Instruction, ParseError> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::push_instr => {
            let source = inner.into_inner().next().unwrap();
            let value = parse_source(source)?;
            Ok(Push(match value {
                Value::Immediate(n) => n,
                _ => return Err(ParseError::InvalidInstruction("Push requires immediate value".into())),
            }))
        },
        Rule::pop_instr => Ok(Pop),
        Rule::dup_instr => Ok(Dup),
        Rule::swap_instr => Ok(Swap),
        Rule::mov_instr => {
            let mut inner = inner.into_inner();
            let src = parse_source(inner.next().unwrap())?;
            let dst = parse_destination(inner.next().unwrap())?;
            Ok(Move { from: src, to: dst })
        },
        Rule::binary_instr => {
            let mut inner = inner.into_inner();
            let op = parse_binary_op(inner.next().unwrap())?;
            let dst = parse_destination(inner.next().unwrap())?;
            let src = parse_source(inner.next().unwrap())?;
            Ok(Binary {
                op,
                modified: dst,
                consumed: src,
            })
        },
        Rule::read_instr => {
            let dst = inner.into_inner().next().unwrap();
            Ok(Read(parse_destination(dst)?))
        },
        Rule::print_instr => {
            let src = inner.into_inner().next().unwrap();
            Ok(Print(parse_source(src)?))
        },
        Rule::match_instr => {
            let mut inner = inner.into_inner();
            let source = parse_destination(inner.next().unwrap())?;

            let mut cases = Vec::new();
            let mut default = None;

            for item in inner {
                match item.as_rule() {
                    Rule::case_block => {
                        let mut case_inner = item.into_inner();
                        let num = case_inner.next().unwrap().as_str()
                            .parse()
                            .map_err(|_| ParseError::InvalidInstruction("Invalid case number".into()))?;
                        let instructions = parse_pairs(case_inner)?;
                        cases.push((num, instructions.into()));
                    },
                    Rule::default_block => {
                        let instructions = parse_pairs(item.into_inner())?;
                        default = Some(instructions.into());
                    },
                    _ => {}
                }
            }

            Ok(Match {
                source,
                cases,
                default: default.unwrap_or_else(|| vec![].into()),
            })
        },
        Rule::while_instr => {
            let mut inner = inner.into_inner();
            let source = parse_destination(inner.next().unwrap())?;
            let body = parse_pairs(inner)?;

            Ok(While {
                source,
                body: body.into(),
            })
        },
        _ => Err(ParseError::InvalidInstruction(format!("Unknown instruction: {:?}", inner)))
    }
}

fn parse_source(pair: Pair<Rule>) -> Result<Value, ParseError> {
    let rule = pair.as_rule();
    match rule {
        Rule::number => Ok(Value::Immediate(
            pair.as_str().parse().map_err(|_| ParseError::InvalidInstruction("Invalid number".into()))?
        )),
        Rule::char_literal => Ok(Value::Immediate(
            pair.as_str().chars().nth(1).unwrap() as u8
        )),
        Rule::variable => Ok(Value::Location(Location::Variable(
            pair.as_str()[1..].parse().map_err(|_| ParseError::InvalidInstruction("Invalid variable".into()))?
        ))),
        Rule::stack => Ok(Value::Location(Location::Stack)),
        _ => Err(ParseError::InvalidInstruction("Invalid source".into()))
    }
}

fn parse_destination(pair: Pair<Rule>) -> Result<Location, ParseError> {
    match pair.as_rule() {
        Rule::variable => Ok(Location::Variable(
            pair.as_str()[1..].parse().map_err(|_| ParseError::InvalidInstruction("Invalid variable".into()))?
        )),
        Rule::stack => Ok(Location::Stack),
        _ => Err(ParseError::InvalidInstruction("Invalid destination".into()))
    }
}

fn parse_binary_op(pair: Pair<Rule>) -> Result<BinaryOp, ParseError> {
    match pair.as_str() {
        "add" => Ok(BinaryOp::Add),
        "sub" => Ok(BinaryOp::Sub),
        "mul" => Ok(BinaryOp::Mul),
        "div" => Ok(BinaryOp::Div),
        "eq" => Ok(BinaryOp::Eq),
        _ => Err(ParseError::InvalidInstruction("Invalid binary operation".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_instructions() {
        let input = "
            # Stack operations
            push 42
            pop
            dup
            swap

            # Move operations
            # From => To
            mov 123, $0
            mov stack, $1
            mov $2, stack

            # Binary operations
            add stack, 5
            sub $0, 1
            mul stack, $1
            div $2, 10
            eq stack, stack
        ";

        let parsed = parse(input).unwrap();
        let correct: Instructions = vec![
            Push(42),
            Pop,
            Dup,
            Swap,
            Move { from: Value::Immediate(123), to: Location::Variable(0) },
            Move { from: Value::Location(Location::Stack), to: Location::Variable(1) },
            Move { from: Value::Location(Location::Variable(2)), to: Location::Stack },
            Binary { op: BinaryOp::Add, modified: Location::Stack, consumed: Value::Immediate(5) },
            Binary { op: BinaryOp::Sub, modified: Location::Variable(0), consumed: Value::Immediate(1) },
            Binary { op: BinaryOp::Mul, modified: Location::Stack, consumed: Value::Location(Location::Variable(1)) },
            Binary { op: BinaryOp::Div, modified: Location::Variable(2), consumed: Value::Immediate(10) },
            Binary { op: BinaryOp::Eq, modified: Location::Stack, consumed: Value::Location(Location::Stack) },
        ].into();
        println!("{:?}", parsed);
        assert_eq!(parsed, correct);
    }

    #[test]
    fn test_io_instructions() {
        let input = r#"
            read stack
            read $0
            print 'A'
            print $1
            print stack
        "#;

        let parsed = parse(input).unwrap();
        let correct: Instructions = vec![
            Read(Location::Stack),
            Read(Location::Variable(0)),
            Print(Value::Immediate(b'A')),
            Print(Value::Location(Location::Variable(1))),
            Print(Value::Location(Location::Stack)),
        ].into();
        println!("{:?}", parsed);
        assert_eq!(parsed, correct);
    }

    #[test]
    fn test_control_flow() {
        let input = r#"
            # Match statement
            match stack
            case 0
                print '0'
            case 1
                push 1
                print stack
            default
                print 'E'
            end

            # While loop
            while $0
                print $0
                sub $0, 1
            end
        "#;

        let parsed = parse(input).unwrap();
        let correct: Instructions = vec![
            Match {
                source: Location::Stack,
                cases: vec![
                    (0, vec![Print(Value::Immediate(b'0'))].into()),
                    (1, vec![Push(1), Print(Value::Location(Location::Stack))].into()),
                ],
                default: vec![Print(Value::Immediate(b'E'))].into(),
            },
            While {
                source: Location::Variable(0),
                body: vec![
                    Print(Value::Location(Location::Variable(0))),
                    Binary {
                        op: BinaryOp::Sub,
                        modified: Location::Variable(0),
                        consumed: Value::Immediate(1),
                    },
                ].into(),
            },
        ].into();
        assert_eq!(parsed, correct);
    }

    #[test]
    fn test_cat_program() {
        let input = r#"
            mov 1, $0
            while $0
                read stack
                dup
                match stack
                case 0
                    mov 0, $0
                default
                    print stack
                end
            end
        "#;

        let parsed = parse(input).unwrap();
        let correct: Instructions = vec![
            Move { from: Value::Immediate(1), to: Location::Variable(0) },
            While {
                source: Location::Variable(0),
                body: vec![
                    Read(Location::Stack),
                    Dup,
                    Match {
                        source: Location::Stack,
                        cases: vec![
                            (0, vec![Move { from: Value::Immediate(0), to: Location::Variable(0) }].into()),
                        ],
                        default: vec![Print(Value::Location(Location::Stack))].into(),
                    },
                ].into(),
            },
        ].into();
        
        assert_eq!(parsed, correct);
    }
}
