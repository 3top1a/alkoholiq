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
    ///
    /// Make sure to zero out the variable if it's in a loop
    Read(Variable),

    /// Print variable to STDOUT
    Print(Variable),

    /// Add variable `b` to variable `a`
    ///
    /// Equivalent to `a += b`.
    Add { a: Variable, b: Variable },

    /// Subtract variable `b` from variable `a`
    ///
    /// Equivalent to `a -= b`.
    Sub { a: Variable, b: Variable },

    // TODO Mul Div
    /// Execute code only if `a` equals `b`
    IfEqual { a: Variable, b: Variable },

    /// Execute code only if `a` does not equal `b`
    IfNotEqual { a: Variable, b: Variable },

    /// Execute code until `a` equals `b`
    UntilEqual { a: Variable, b: Variable },

    /// Execute code while `a` is not zero
    WhileNotZero(Variable),

    /// End clause to end if/until blocks
    End,

    /// Compare two variables, `a` and `b`, and store result into `res`
    ///
    /// Third variable is the result of the comparison:
    /// - 0 if equal
    /// - 1 if a < b
    /// - 2 if a > b
    Compare {
        a: Variable,
        b: Variable,
        res: Variable,
    },

    /// Insert raw brainfuck
    ///
    /// Only use if you have to, must put pointer back into position after every use
    Raw(String),
}
