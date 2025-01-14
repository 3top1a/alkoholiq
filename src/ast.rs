#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // Literals
    /// Numeric literal
    /// Doubles as a ASCII character literal, for Unicode an Array of these is used instead.
    /// Also used for boolean literals, where 0 is false and anything non-zero is true.
    Number(u8),

    /// Array
    Array(Vec<Expression>),

    /// A literal identifier
    Identifier(String),

    /// Basic arithmetic operation
    Arithmetic {
        /// Left side
        left: Box<Expression>,
        /// Operator
        op: MathOperator,
        /// Right side
        right: Box<Expression>,
    },

    /// Assignment
    Assignment {
        /// Name of the variable
        name: String,
        /// Value to assign
        value: Box<Expression>,
    },

    /// Closure
    Closure {
        /// Body
        body: Vec<Expression>,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
