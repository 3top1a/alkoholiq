use crate::lir::lir::{Instruction, Variable};
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
}

impl InstructionsParsed {
    pub fn new(instructions: Vec<Instruction>) -> Result<Self> {
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
        // As there are two temporary variables in the front, index starts at 2
        let mut index = 2;

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
                Instruction::Copy { a, b } => {
                    var(a)?;
                    var(b)?
                }
                Instruction::Inc(a) => var(a)?,
                Instruction::Dec(a) => var(a)?,
                Instruction::Set(a, ..) => var(a)?,
                Instruction::Read(a) => var(a)?,
                Instruction::Print(a) => var(a)?,
                Instruction::Add { a, b } => {
                    var(a)?;
                    var(b)?
                }
                Instruction::Sub { a, b } => {
                    var(a)?;
                    var(b)?
                }
                Instruction::Raw(_) => {}
            }
        }

        Ok(variables)
    }
}
