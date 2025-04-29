// parser/mod.rs
use crate::lir::lir::Instruction;
use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "lir/grammar.pest"]
struct LirParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(pest::error::Error<Rule>),
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {
    let pairs = LirParser::parse(Rule::program, input)?;
    let mut instructions = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for instruction in pair.into_inner() {
                    if let Some(inst) = parse_instruction(instruction)? {
                        instructions.push(inst);
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(instructions)
}

fn parse_instruction(pair: Pair<Rule>) -> Result<Option<Instruction>> {
    match pair.as_rule() {
        Rule::instruction => {
            let inner = pair.into_inner().next().unwrap();
            Ok(Some(match inner.as_rule() {
                Rule::copy_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    Instruction::Copy { a, b }
                }
                Rule::inc_instr => {
                    let var = inner.into_inner().next().unwrap().as_str().to_string();
                    Instruction::Inc(var)
                }
                Rule::dec_instr => {
                    let var = inner.into_inner().next().unwrap().as_str().to_string();
                    Instruction::Dec(var)
                }
                Rule::inc_by_instr => {
                    let mut inner = inner.into_inner();
                    let var = inner.next().unwrap().as_str().to_string();
                    let val = inner.next().unwrap().as_str().parse().unwrap();
                    Instruction::IncBy(var, val)
                }
                Rule::dec_by_instr => {
                    let mut inner = inner.into_inner();
                    let var = inner.next().unwrap().as_str().to_string();
                    let val = inner.next().unwrap().as_str().parse().unwrap();
                    Instruction::DecBy(var, val)
                }
                Rule::set_instr => {
                    let mut inner = inner.into_inner();
                    let var = inner.next().unwrap().as_str().to_string();
                    let val = inner.next().unwrap().as_str().parse().unwrap();
                    Instruction::Set(var, val)
                }
                Rule::read_instr => {
                    let var = inner.into_inner().next().unwrap().as_str().to_string();
                    Instruction::Read(var)
                }
                Rule::print_instr => {
                    let var = inner.into_inner().next().unwrap().as_str().to_string();
                    Instruction::Print(var)
                }
                Rule::print_msg_instr => {
                    let msg = inner.into_inner().next().unwrap().as_str();
                    // Remove quotes from string literal
                    let msg = msg[1..msg.len() - 1].to_string();
                    Instruction::PrintMsg(msg)
                }
                Rule::add_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    Instruction::Add { a, b }
                }
                Rule::sub_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    Instruction::Sub { a, b }
                }
                Rule::if_equal_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    Instruction::IfEqual { a, b }
                }
                Rule::if_equal_const_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().parse().unwrap();
                    Instruction::IfEqualConst { a, b }
                }
                Rule::if_not_equal_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    Instruction::IfNotEqual { a, b }
                }
                Rule::until_equal_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    Instruction::UntilEqual { a, b }
                }
                Rule::while_not_zero_instr => {
                    let var = inner.into_inner().next().unwrap().as_str().to_string();
                    Instruction::WhileNotZero(var)
                }
                Rule::compare_instr => {
                    let mut inner = inner.into_inner();
                    let a = inner.next().unwrap().as_str().to_string();
                    let b = inner.next().unwrap().as_str().to_string();
                    let res = inner.next().unwrap().as_str().to_string();
                    Instruction::Compare { a, b, res }
                }
                Rule::raw_instr => {
                    let raw = inner.into_inner().next().unwrap().as_str();
                    // Remove quotes from string literal
                    let raw = raw[1..raw.len() - 1].to_string();
                    Instruction::Raw(raw)
                }
                Rule::end_instr => Instruction::End,
                _ => return Err(ParseError::InvalidInstruction(inner.as_str().to_string()).into()),
            }))
        }
        Rule::EOI => Ok(None),
        _ => Err(ParseError::InvalidInstruction(pair.as_str().to_string()).into()),
    }
}
