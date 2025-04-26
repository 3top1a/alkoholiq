use crate::lir::instructions::InstructionsParsed;

pub type Immediate = u8;
pub type Variable = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Copy from variable a to variable b
    Copy { a: Variable, b: Variable },

    /// Increment variable by one
    Inc(Variable),
    /// Decrement variable by one
    Dec(Variable),
    /// Increment variable by value
    IncBy(Variable, Immediate),
    /// Decrement variable by value
    DecBy(Variable, Immediate),

    /// Set variable to value
    Set(Variable, Immediate),

    /// Read STDIN into variable
    Read(Variable),

    /// Print variable to STDOUT
    Print(Variable),

    /// Add variable `b` to variable `a`
    Add { a: Variable, b: Variable },

    /// Subtract variable `b` from variable `a`
    Sub { a: Variable, b: Variable },

    /// Raw brainfuck
    /// Only use if you have to, must put pointer back into position after every use
    Raw(String),
}
