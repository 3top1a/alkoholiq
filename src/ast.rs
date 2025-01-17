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

    // Now also a function
    // /// Unary arithmetic operation
    // Unary {
    //     Operator
    // op: UnaryOperator,
    // Operand
    // operand: Box<Expression>,
    // },

    // Not used right now because maths are functions
    // /// Basic arithmetic operation
    // /// Since the output is a number anyway, we can group comparisons with arithmetic operations.
    // Binary {
    //     /// Left side
    //     lhs: Box<Expression>,
    //     /// Operator
    //     op: MathOperator,
    //     /// Right side
    //     rhs: Box<Expression>,
    // },

    /// Variable assignment
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
    },

    /// For loop
    For {
        /// Variable name
        name: String,
        /// Range
        range: AlcIterator,
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

    // Function
    Function {
        /// Name of the function
        name: String,
        /// Arguments
        args: Vec<String>,
        /// Body
        body: Box<Expression>,
    },
}

// Not used right now because maths are functions
// #[derive(Debug, PartialEq, Clone)]
// pub enum MathOperator {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     Equals,
//     NotEquals,
//     LessThan,
//     GreaterThan,
// }

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// Negate a number
    /// Works by subtracting the number from 0 and returning the absolute value.
    Negate,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AlcIterator {
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    Variable(Box<Expression>),
}

impl Expression {
    pub fn as_number(&self) -> Option<u8> {
        match self {
            Expression::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Expression::Number(_) => 1,
            Expression::Array(arr) => arr.iter().map(|e| e.size()).sum(),
            _ => 0,
        }
    }
}
