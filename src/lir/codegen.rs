use crate::lir::instructions::Instructions;
use crate::lir::lir::Instruction::*;
use crate::lir::lir::{Location, Value};

#[derive(Debug, Clone)]
pub struct Codegen {
    code: String,
    variables: usize,
    ptr: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            variables: 0,
            ptr: 0,
        }
    }

    /// Generate code from instructions.
    ///
    /// This consumes the Codegen instance and returns the generated code.
    pub fn codegen(mut self, instructions: Instructions) -> String {
        self.variables = instructions.get_variable_indexes().len();

        // Allocate memory for variables
        // One less because stack ops go right once
        let number_left = (self.variables - 1).min(0);
        self.code += ">".repeat(number_left).as_str();
        self.code += "\n";
        self.ptr = number_left;

        for instr in instructions.0 {
            self.code += format!("{}", instr.debug()).as_str();

            match instr {
                Push(n) => self.push(n),
                Pop => self.pop(),
                Dup => self.dup(),

                Copy { from, to } => self.copy(from, to),

                _ => unimplemented!(),
            }

            self.code += "\n";
        }

        self.code
    }

    fn goto_stack(&mut self) {
        // TODO
    }

    fn push(&mut self, n: u8) {
        self.goto_stack();
        self.code += ">";
        self.code += "+".repeat(n as usize).as_str();
        self.ptr += 1;
    }

    fn pop(&mut self) {
        self.goto_stack();
        self.code += "[-]<";
        self.ptr -= 1;
    }

    fn dup(&mut self) {
        self.goto_stack();
        self.code += "[->+>+<<]>>[-<<+>>]<";
        self.ptr += 1;
    }

    fn copy(&mut self, from: Value, to: Location) {
        match from {
            Value::Immediate(n) => {
                match to {
                    Location::Stack => {
                        self.push(n);
                    }
                    Location::Variable(var) => {
                        // Goto var
                        self.code += "<".repeat(self.ptr - var).as_str();
                        // Add n
                        // TODO Optim for n = 0
                        self.code += "[-]+".repeat(n as usize).as_str();
                        // Go back
                        self.code += ">".repeat(self.ptr - var).as_str();
                    }
                }
            }
            Value::Location(loc) => {
                match loc {
                    Location::Stack => {
                        match to {
                            Location::Stack => {
                                unimplemented!("Are you dump")
                            }
                            Location::Variable(var) => {
                                // [- << + >> ]<
                                self.code += "[-";
                                self.code += "<".repeat(self.ptr - var).as_str();
                                self.code += "+";
                                self.code += ">".repeat(self.ptr - var).as_str();
                                self.code += "]<";
                                self.ptr -= 1;
                            }
                        }
                    }
                    Location::Variable(_) => {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
