pub type Immediate = u8;
pub type Variable = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Copy from variable a to variable b, zeroing a
    Copy { a: Variable, b: Variable },

    /// Set variable to value
    Set(Variable, Immediate),

    /// Increment variable by one
    Inc(Variable),
    /// Decrement variable by one
    Dec(Variable),
    /// Increment variable by value
    IncBy(Variable, Immediate),
    /// Decrement variable by value
    DecBy(Variable, Immediate),

    /// Read STDIN into variable
    ///
    /// Make sure to zero out the variable if it's in a loop
    Read(Variable),

    /// Print variable to STDOUT
    Print(Variable),

    /// Print a string to STDOUT
    ///
    /// Includes escapes for newlines, carriage return and tabs
    PrintMsg(String),

    /// Print variable as a human-readable number
    ///
    /// E.g. 0x10 will print as `10`
    PrintC(Variable),

    /// Add variable `b` to variable `a`
    ///
    /// Equivalent to `a += b`.
    Add { a: Variable, b: Variable },

    /// Subtract variable `b` from variable `a`
    ///
    /// Equivalent to `a -= b`.
    Sub { a: Variable, b: Variable },

    /// Multiply two variables
    ///
    /// Equivalent to `a *= b`.
    Mul { a: Variable, b: Variable },

    /// Divide two variables, storing quotient in `q` and remainder in `r`
    ///
    /// Equivalent to `q = a / b; r = a % b`.
    Div {
        a: Variable,
        b: Variable,
        quotient: Variable,
        remainder: Variable,
    },

    /// Execute code only if `a` equals `b`
    ///
    /// All If and While loops CANNOT touch any temporary variables after the body ends.
    /// They must also end at the same position no matter if the condition is true or false.
    IfEqual { a: Variable, b: Variable },

    /// Execute code only if `a` equals constant
    IfEqualConst { a: Variable, b: Immediate },

    /// Execute code only if `a` doesn't equal constant
    IfNotEqualConst { a: Variable, b: Immediate },

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

    /// Push variable onto stack
    Push(Variable),

    /// Pop variable from stack
    ///
    /// Warning: Does not check for stack underflow and other undefined behavior
    Pop(Variable),

    /// Match value of a variable
    /// 
    /// Second argument needs to be sorted in ascending order, and the cases instructions must be
    /// in the *exact opposite* order of the values in the second argument.
    Match(Variable, Vec<Immediate>),

    /// Case in a match case
    Case(),

    /// Insert raw brainfuck
    ///
    /// Only use if you have to, must put pointer back into position after every use
    Raw(String),
}
