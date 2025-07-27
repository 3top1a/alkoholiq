use crate::lir::instruction::{Instruction, Instruction::*, Variable};
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InstructionsAnalysis {
    instructions: Vec<Instruction>,
    pub variables: HashMap<String, i32>,
    pub variable_count: i32,
}

#[derive(Debug, Clone, Error)]
enum InstructionError {
    #[error("Invalid variable name: {v}")]
    InvalidVariableName { v: Variable },

    #[error("Variable {v} must be assigned before use")]
    VariableMustBeAssigned { v: Variable },

    #[error("Uneven amount of blocks")]
    UnevenAmountOfBlocks(),
}

impl InstructionsAnalysis {
    pub fn new(instructions: Vec<Instruction>) -> Result<Self> {
        Self::sanity_check(instructions.clone())?;

        let (variables, variable_count) = Self::build_variable_hashmap(instructions.clone())?;

        Ok(Self {
            instructions,
            variables,
            variable_count,
        })
    }

    fn build_variable_hashmap(input: Vec<Instruction>) -> Result<(HashMap<String, i32>, i32)> {
        let mut variables = HashMap::new();
        let mut index = 0;

        let mut var = |v: Variable, must_be_defined: bool| {
            if must_be_defined && !variables.contains_key(&v) {
                return Err(InstructionError::VariableMustBeAssigned { v });
            }

            if variables.contains_key(&v) {
                return Ok(());
            }

            // Variables must start with a letter and can contain alpha, _-, and digits
            if !v.chars().next().unwrap().is_alphabetic() {
                return Err(InstructionError::InvalidVariableName { v });
            }
            if !v
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                return Err(InstructionError::InvalidVariableName { v });
            }

            variables.insert(v, index);
            index += 1;
            Ok(())
        };

        // Register and check validity of variable accesses
        for i in input {
            match i {
                Copy { a, b } => {
                    var(a, true)?;
                    var(b, false)?
                }
                Inc(a) => var(a, false)?,
                Dec(a) => var(a, false)?,
                IncBy(a, ..) => var(a, false)?,
                DecBy(a, ..) => var(a, false)?,
                Set(a, ..) => var(a, false)?,
                Read(a) => var(a, false)?,
                Print(a) => var(a, true)?,
                PrintC(a) => var(a, true)?,
                Add { a, b } => {
                    var(a, true)?;
                    var(b, true)?
                }
                Sub { a, b } => {
                    var(a, true)?;
                    var(b, true)?
                }
                IfEqual { a, b } => {
                    var(a, true)?;
                    var(b, true)?
                }
                IfEqualConst { a, .. } => {
                    var(a, true)?;
                }
                IfNotEqual { a, b } => {
                    var(a, true)?;
                    var(b, true)?
                }
                IfNotEqualConst { a, .. } => {
                    var(a, true)?;
                }
                UntilEqual { a, b } => {
                    var(a, true)?;
                    var(b, true)?
                }
                WhileNotZero(a) => var(a, true)?,
                End => {}
                Raw(_) => {}
                Compare { a, b, res } => {
                    var(a, true)?;
                    var(b, true)?;
                    var(res, false)?
                }
                PrintMsg(_) => {}
                Mul { a, b } => {
                    var(a, true)?;
                    var(b, true)?
                }
                Div {
                    a,
                    b,
                    remainder: r,
                    quotient: q,
                } => {
                    var(a, true)?;
                    var(b, true)?;
                    var(r, false)?;
                    var(q, false)?;
                }
                Push(a) => {
                    var(a, true)?;
                }
                Pop(a) => {
                    var(a, false)?;
                }
                Match(a, _) => var(a, true)?,
                Case() => {},
            }
        }

        // Register temporary variables

        // variables.insert("1".to_string(), -2);
        // variables.insert("0".to_string(), -1);
        for i in 0..17 {
            variables.insert(i.to_string(), -i - 1);
        }

        Ok((variables, index))
    }

    fn sanity_check(instructions: Vec<Instruction>) -> Result<()> {
        let mut nesting = 0i32;
        for i in instructions {
            match i {
                IfEqual { .. }
                | IfNotEqual { .. }
                | UntilEqual { .. }
                | WhileNotZero(..)
                | IfNotEqualConst { .. }
                | IfEqualConst { .. }
                | Match(..) => nesting += 1,
                End => nesting -= 1,
                _ => {}
            }
        }

        if nesting != 0 {
            return Err(InstructionError::UnevenAmountOfBlocks().into());
        }

        Ok(())
    }
}
