/// Low Intermediate Representation
///
/// A smaller abstraction on top of brainfuck that turns it into a stack machine
#[derive(Debug, Clone, Copy)]
pub enum LIR {
    /// Push a value on top of the stack
    Push(u8),
    /// Remove the topmost value
    Pop,
    /// Push a 0 on top of the stack
    False,
    /// Push a 1 on top of the stack
    True,

    /// Copy the value at stack number x and push it to the top
    Var(usize),
    /// Duplicate the top value
    Dup,

    /// Prints the top value while poping it
    Print,
    /// Take one byte from stdin and push it
    Input,

    /// Add the two topmost values
    Add,
    /// Substract the two topmost values
    /// Can also be used as a not equals operation
    Sub,
    // MUL,
    /// Check if the two topmost values are equal
    /// Pushes 1 if true, 0 if false
    Eq,

    /// Start of an if block without an else block
    ///
    /// If the top value is 0, skip the next instruction.
    /// This will consume the top value either way.
    StartIf,
    StartElse,
    /// End of an if block
    EndIf,
}
