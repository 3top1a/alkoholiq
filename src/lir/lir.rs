use crate::lir::instructions::InstructionsParsed;

pub type Immediate = u8;
pub type Variable = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Copy from variable a to variable b
    Copy {
        a: Variable,
        b: Variable,
    },

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
    ///
    /// Equivalent to `a += b`.
    Add {
        a: Variable,
        b: Variable,
    },

    /// Subtract variable `b` from variable `a`
    ///
    /// Equivalent to `a -= b`.
    Sub {
        a: Variable,
        b: Variable,
    },

    // TODO Mul Div
    /// Execute code only if `a` equals `b`
    IfEqual {
        a: Variable,
        b: Variable,
    },

    IfNotEqual {
        a: Variable,
        b: Variable,
    },

    UntilEqual {
        a: Variable,
        b: Variable,
    },

    WhileNotZero(Variable),

    /// End clause to end if/until blocks
    End,

    /// Insert raw brainfuck
    ///
    /// Only use if you have to, must put pointer back into position after every use
    Raw(String),
}
