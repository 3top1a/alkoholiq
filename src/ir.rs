use crate::ast::Expression;
use std::collections::HashMap;


/// Intermediate representation
/// The main purpuse of this is to convert the nested expression into a linear flat instruction list
/// that is more similar to the way brainfuck works
///
/// Resources:
/// - https://andreabergia.com/series/stack-based-virtual-machines/
#[derive(Clone, Debug)]
pub enum IR {
    /// Push a single byte on the stack
    Push(u8),
    /// Pop a value from the stack (zero it out)
    Pop,

    /// Add the two top most values
    ADD,
    /// Subtract the two topmost values
    SUB,
    /// Multiply the two topmost values
    MUL,
    /// Check for equality of the two topmost values
    EQ,

    /// Duplicate the topmost value
    DUP,

    /// Push topmost byte to variable storage
    STORE(String),
    /// Retrieve a named value
    /// This removes the variable from the storage
    LOAD(String),
    /// Start if block
    START_IF,
    /// End if block
    CLOSE_IF,

    // TODO Else blocks
    // TODO Functions and frames
}

#[derive(Clone, Debug)]
pub struct IRGenerator {
    ir: Vec<IR>,
    variables: HashMap<String, usize>
}

impl IRGenerator {
    pub fn new() -> IRGenerator {
        Self {
            ir: vec![],
            variables: HashMap::new(),
        }
    }

    pub fn generate(&mut self, ast: Vec<Expression>) -> Vec<IR> {
        for expr in ast {
            self.generate_expression(expr);
        }

        self.ir.clone()
    }

    fn load(&mut self, expr: Expression) {
        match expr {
            Expression::Number(n) => {
                self.ir.push(IR::Push(n));
            }

            Expression::Path(path) => {
                // Duplicate the value
                self.ir.push(IR::LOAD(path.clone()));
                self.ir.push(IR::DUP);
                self.ir.push(IR::STORE(path));
            }

            _ => {
                todo!("Not implemented: {:?}", expr);
            }
        }
    }

    fn generate_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Number(n) => {
                self.ir.push(IR::Push(n));
            }

            Expression::Closure {body} => {
                for expr in body {
                    self.generate_expression(expr);
                }
            }

            Expression::Assignment {name, value} => {
                self.generate_expression(*value);

                self.variables.insert(name.clone(), 0);
                self.ir.push(IR::STORE(name));
            }

            Expression::Call {name, args} => {
                // Load all arguments
                for arg in args {
                    self.load(arg);
                }

                if name == "+" {
                    self.ir.push(IR::ADD);
                } else if name == "-" {
                    self.ir.push(IR::SUB);
                } else if name == "*" {
                    self.ir.push(IR::MUL);
                } else if name == "==" {
                    self.ir.push(IR::EQ);
                } else {
                    todo!("Not implemented: {:?}", name);
                }
            }

            _ => {
                dbg!("Not implemented:", expression);
            }
        }
    }
}

