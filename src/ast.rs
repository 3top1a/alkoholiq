#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Numeric literal
    /// Doubles as an ASCII character literal, for Unicode an Array of these is used instead.
    /// Also used for boolean literals, where 0 is false and 1 is true.
    Number(u8),
    /// Array literal
    Array(Vec<Expression>),

    /// A path denotes a variable
    Path(String),

    /// A function call
    Call {
        /// Name of the function
        name: String,
        /// Arguments
        args: Vec<Expression>,
    },

    /// Unary arithmetic operation
    Unary {
        /// Operator
        op: UnaryOperator,
        /// Operand
        operand: Box<Expression>,
    },

    /// Basic arithmetic operation
    /// Since the output is a number anyway, we can group comparisons with arithmetic operations.
    Binary {
        /// Left side
        lhs: Box<Expression>,
        /// Operator
        op: MathOperator,
        /// Right side
        rhs: Box<Expression>,
    },

    /// Variable assignment
    Assignment {
        /// Name of the variable
        name: String,
        /// Value to assign
        value: Box<Expression>,
    },

    /// Closure
    Expression {
        /// Body
        body: Vec<Expression>,
    },

    /// For loop
    For {
        /// Variable name
        name: String,
        /// Range
        range: Iterator,
        /// Body
        /// Should be of type Expression::Expression
        body: Box<Expression>,
    },

    // If
    If {
        /// Condition
        condition: Box<Expression>,
        /// Then branch
        /// Should be of type Expression::Expression
        then_branch: Box<Expression>,
        /// Else branch
        else_branch: Option<Box<Expression>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// Negate a number
    /// Works by subtracting the number from 0 and returning the absolute value.
    Negate,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Iterator {
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    Path(Box<Expression>),
}
