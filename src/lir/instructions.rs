use crate::lir::lir::{Instruction, Location, Value};

#[derive(Debug, Clone)]
pub struct Instructions(pub Vec<Instruction>);

impl Instructions {
    /// Validates instructions and panics if invalid.
    pub fn validate(&self) -> bool {
        // Check that all variables are declared before use
        let variables = self.get_variable_indexes();
        for instr in &self.0 {
            match instr {
                Instruction::Binary {
                    modified, consumed, ..
                } => {
                    if let Location::Variable(idx) = modified {
                        if !variables.contains(idx) {
                            return false;
                        }
                    }
                    if let Value::Location(Location::Variable(idx)) = consumed {
                        if !variables.contains(idx) {
                            return false;
                        }
                    }
                }
                Instruction::Copy { from, to } => {
                    if let Value::Location(Location::Variable(idx)) = from {
                        if !variables.contains(idx) {
                            return false;
                        }
                    }
                    if let Location::Variable(idx) = to {
                        if !variables.contains(idx) {
                            return false;
                        }
                    }
                }
                Instruction::Read(Location::Variable(idx)) => {
                    if !variables.contains(idx) {
                        return false;
                    }
                }
                Instruction::Match {
                    source: _source,
                    cases,
                    default,
                } => {
                    if cases.iter().map(|x| x.1.validate()).any(|x| !x)
                        || !default.validate()
                    {
                        return false;
                    }
                }
                _ => (),
            }
        }

        true
    }

    pub fn get_variable_indexes(&self) -> Vec<usize> {
        // Gather all variable indexes into Vec
        let mut variables = Vec::new();
        for instr in &self.0 {
            match instr {
                Instruction::Binary {
                    modified, consumed, ..
                } => {
                    if let Location::Variable(idx) = modified {
                        variables.push(*idx);
                    }
                    if let Value::Location(Location::Variable(idx)) = consumed {
                        variables.push(*idx);
                    }
                }
                Instruction::Copy { from, to } => {
                    if let Value::Location(Location::Variable(idx)) = from {
                        variables.push(*idx);
                    }
                    if let Location::Variable(idx) = to {
                        variables.push(*idx);
                    }
                }
                Instruction::Read(Location::Variable(idx)) => {
                    variables.push(*idx);
                }
                Instruction::Match {
                    source: _source,
                    cases,
                    default,
                } => {
                    variables.extend(default.get_variable_indexes());
                    let vars = cases
                        .iter()
                        .map(|x| x.1.get_variable_indexes())
                        .reduce(|acc, x| [acc, x].concat())
                        .unwrap();
                    variables.extend(vars);
                }
                _ => (),
            }
        }

        // Return unique count
        variables.sort();
        variables.dedup();
        variables
    }
}

// From Vec<Instruction> to Instructions
impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        Instructions(instructions)
    }
}
