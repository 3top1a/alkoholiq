use crate::lir::lir::{Instruction, Instruction::*, Variable};
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct InstructionsParsed {
    instructions: Vec<Instruction>,
    pub variables: HashMap<String, i32>,
}

impl Default for InstructionsParsed {
    fn default() -> Self {
        InstructionsParsed {
            instructions: Vec::new(),
            variables: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Error)]
enum InstructionError {
    #[error("Invalid variable name: {v}")]
    InvalidVariableName { v: Variable },

    #[error("Uneven amount of blocks")]
    UnevenAmountOfBlocks(),
}

impl InstructionsParsed {
    pub fn new(instructions: Vec<Instruction>) -> Result<Self> {
        Self::sanity_check(instructions.clone())?;

        let variables = Self::build_variable_hashmap(instructions.clone())?;

        Ok(Self {
            instructions,
            variables,
        })
    }

    fn build_variable_hashmap(input: Vec<Instruction>) -> Result<HashMap<String, i32>> {
        let mut variables = HashMap::new();
        // Register the first two temp vars
        variables.insert("0".to_string(), 0);
        variables.insert("1".to_string(), 1);
        variables.insert("2".to_string(), 2);
        // As there are temporary variables in the front, index starts at 3
        let mut index = 3;

        let mut var = |v: Variable| {
            if variables.contains_key(&v) {
                return Ok(());
            }

            if !v.chars().all(char::is_alphabetic) {
                return Err(InstructionError::InvalidVariableName { v });
            }
            variables.insert(v, index);
            index += 1;
            Ok(())
        };

        for i in input {
            match i {
                Copy { a, b } => {
                    var(a)?;
                    var(b)?
                }
                Inc(a) => var(a)?,
                Dec(a) => var(a)?,
                IncBy(a, ..) => var(a)?,
                DecBy(a, ..) => var(a)?,
                Set(a, ..) => var(a)?,
                Read(a) => var(a)?,
                Print(a) => var(a)?,
                Add { a, b } => {
                    var(a)?;
                    var(b)?
                }
                Sub { a, b } => {
                    var(a)?;
                    var(b)?
                }
                IfEqual { a, b } => {
                    var(a)?;
                    var(b)?
                }
                IfNotEqual { a, b } => {
                    var(a)?;
                    var(b)?
                }
                UntilEqual { a, b } => {
                    var(a)?;
                    var(b)?
                }
                WhileNotZero(a) => var(a)?,
                End => {}
                Raw(_) => {}
            }
        }

        Ok(variables)
    }

    fn sanity_check(instructions: Vec<Instruction>) -> Result<()> {
        let mut nesting = 0i32;
        for i in instructions {
            match i {
                IfEqual { .. } | IfNotEqual { .. } | UntilEqual { .. } | WhileNotZero(..) => {
                    nesting += 1
                }
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
