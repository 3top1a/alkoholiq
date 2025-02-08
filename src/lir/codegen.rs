use crate::lir::instructions::Instructions;
use crate::lir::lir::Instruction::*;
use crate::lir::lir::{BinaryOp, Location, Value};

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
        instructions.validate();

        self.variables = instructions.get_variable_indexes().len();

        // Allocate memory for variables
        // One less because stack ops go right once
        let number_left = self.variables.checked_sub(1).unwrap_or(0);
        self.code += format!("Variables: {} ", number_left).as_str();
        self.code += ">".repeat(number_left).as_str();
        self.code += "\n";
        self.ptr = number_left;

        for instr in instructions.0 {
            self.code += format!("{} ", instr.debug()).as_str();

            match instr {
                Push(n) => self.push(n),
                Pop => self.pop(),
                Dup => self.dup(),
                Swap => self.swap(),
                Binary {
                    op,
                    modified,
                    consumed,
                } => self.binary(op, modified, consumed),
                Copy { from, to } => self.copy(from, to),
                Read(loc) => self.read(loc),
                Print(val) => self.print(val),

                Match(loc) => self.startmatch(loc),
                EndCaseDefault => self.endcasedefault(),
                CaseDefault => self.casedefault(),
                Case(n) => self.case(n),
                EndCase => self.endcase(),
                EndMatch => self.endmatch(),

                // _ => unimplemented!("Instruction not implemented: {:?}", instr),
            }

            self.code += "\n";
        }

        self.code
    }

    fn goto_stack(&mut self) {
        // Ensure we're at the top of the stack
        // That means pointer at the last populated cell (my drunk future self)
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

    fn swap(&mut self) {
        self.goto_stack();
        self.code += "[->+<]<[->+<]>>[-<<+>>]<";
    }

    fn copy(&mut self, from: Value, to: Location) {
        // TODO Refactor > Extract Method
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
                        self.code += "+".repeat(n as usize).as_str();
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

    fn binary(&mut self, operation: BinaryOp, modified: Location, consumed: Value) {
        let ops = (modified, consumed);
        match operation {
            BinaryOp::Add => match ops {
                (Location::Stack, Value::Immediate(n)) => {
                    self.goto_stack();
                    self.code += "+".repeat(n as usize).as_str();
                }
                (Location::Stack, Value::Location(Location::Stack)) => {
                    self.goto_stack();
                    self.code += "[-<+>]<";
                }
                _ => unimplemented!(),
            },
            BinaryOp::Sub => match ops {
                (Location::Stack, Value::Immediate(n)) => {
                    self.goto_stack();
                    self.code += "-".repeat(n as usize).as_str();
                }
                (Location::Stack, Value::Location(Location::Stack)) => {
                    self.goto_stack();
                    self.code += "[-<->]<";
                }
                _ => unimplemented!(),
            },
            BinaryOp::Mul => {}
            BinaryOp::Div => {}
            BinaryOp::Eq => {}
        }
    }

    fn read(&mut self, loc: Location) {
        match loc {
            Location::Stack => {
                self.goto_stack();
                self.code += ">,";
            }
            Location::Variable(var) => {
                self.goto_stack(); // So that the math works out
                                   // Goto var
                self.code += "<".repeat(self.ptr - var).as_str();
                // Read
                self.code += ",";
                // Go back
                self.code += ">".repeat(self.ptr - var).as_str();
            }
        }
    }

    fn print(&mut self, val: Value) {
        match val {
            Value::Immediate(n) => {
                self.goto_stack();
                self.push(n);
                self.code += ".[-]<";
            }
            Value::Location(loc) => {
                match loc {
                    Location::Stack => {
                        self.goto_stack();
                        self.code += ".[-]<";
                    }
                    Location::Variable(var) => {
                        // Goto var
                        self.code += "<".repeat(self.ptr - var).as_str();
                        // Print
                        self.code += ".";
                        // Go back
                        self.code += ">".repeat(self.ptr - var).as_str();
                    }
                }
            }
        }
    }

    fn startmatch(&mut self, loc: Location) {
        /*
        For example:

        ++++
        >+<[
            -[
                [-]>-default#<
            ]>[-1#]<
        ]>[-0#]<

        Every iteration we decrement and if it's zero we don't recurse further.

        see https://brainfuck.org/function_tutorial.b
         */
        match loc {
            Location::Stack => {
                self.goto_stack();
                self.code += ">+<[";
                self.ptr += 1;
            }
            _ => unimplemented!(),
        }
    }

    fn endmatch(&mut self) {
        // TODO
    }

    fn case(&mut self, n: u8) {
        // TODO
    }

    fn endcase(&mut self) {
        // TODO
    }

    fn casedefault(&mut self) {
        // TODO
    }

    fn endcasedefault(&mut self) {
        // TODO
    }
}
