use crate::lir::lir::{Instruction, Location, Value};

pub struct Instructions(pub Vec<Instruction>);
impl Instructions {
    /// Validates instructions and returns if it's valid
    pub fn validate(&self) -> bool {
        // Check for equal match and unmatch
        let mut match_count = 0;
        for instr in &self.0 {
            match instr {
                Instruction::Match(_) => match_count += 1,
                Instruction::EndMatch => match_count -= 1,
                _ => (),
            }
        }
        if match_count != 0 {
            return false;
        }

        // Check that case (x) are ordered from 0 to n
        let mut highest_case = 0;
        for instr in &self.0 {
            match instr {
                Instruction::Case(x) => {
                    if *x > highest_case {
                        highest_case = *x;
                    } else {
                        return false;
                    }
                }
                Instruction::Match(_) => highest_case = 0,
                _ => (),
            }
        }

        // Check that all variables are declared before use
        let variables = self.get_variable_indexes();
        for instr in &self.0 {
            match instr {
                Instruction::Binary {
                    modified, consumed, ..
                } => {
                    if let Location::Variable(idx) = modified {
                        if !variables.contains(&idx) {
                            return false;
                        }
                    }
                    if let Value::Location(Location::Variable(idx)) = consumed {
                        if !variables.contains(&idx) {
                            return false;
                        }
                    }
                }
                Instruction::Copy { from, to } => {
                    if let Value::Location(Location::Variable(idx)) = from {
                        if !variables.contains(&idx) {
                            return false;
                        }
                    }
                    if let Location::Variable(idx) = to {
                        if !variables.contains(&idx) {
                            return false;
                        }
                    }
                }
                Instruction::Read(Location::Variable(idx)) => {
                    if !variables.contains(&idx) {
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
