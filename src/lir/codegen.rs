use crate::lir::instructions::InstructionsParsed;
use crate::lir::lir::{Immediate, Instruction, Instruction::*, Variable};
use anyhow::Result;
use std::string::ToString;

#[derive(Debug, Clone)]
enum BlockStack {
    IfEqual { a: Variable, b: Variable },
    IfNotEqual { a: Variable, b: Variable },
    UntilEqual { a: Variable, b: Variable },
    WhileNotZero(),
}

#[derive(Debug, Clone)]
pub struct Codegen {
    code: String,
    ptr: i32,
    pub instructions: Vec<Instruction>,
    parsed: InstructionsParsed,
    block_stack: Vec<BlockStack>,
}

impl Codegen {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            code: String::new(),
            ptr: 0,
            instructions,
            parsed: InstructionsParsed::default(),
            block_stack: Vec::new(),
        }
    }

    pub fn codegen(mut self) -> Result<String> {
        self.parsed = InstructionsParsed::new(self.instructions.clone())?;

        for instruction in self.instructions.clone() {
            self.instruction(instruction)?
        }

        Ok(self.code)
    }

    fn instruction(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            Copy { a, b } => self.copy(&a, &b),
            Inc(a) => self.inc_by(&a, &1),
            Dec(a) => self.dec_by(&a, &1),
            IncBy(a, b) => self.inc_by(&a, &b),
            DecBy(a, b) => self.dec_by(&a, &b),
            Set(a, b) => self.set(&a, &b),
            Read(a) => self.read(&a),
            Print(a) => self.print(&a),
            Add { a, b } => self.add(&a, &b),
            Sub { a, b } => self.sub(&a, &b),
            Raw(raw) => self.code += &*raw,
            IfEqual { a, b } => self.if_equal(&a, &b),
            IfNotEqual { a, b } => self.if_not_equal(&a, &b),
            UntilEqual { a, b } => self.until_equal(&a, &b),
            WhileNotZero(a) => self.while_not_zero(&a),
            End => self.end(),
        }

        Ok(())
    }

    /// Set a variable to a value
    fn set(&mut self, a: &Variable, b: &Immediate) {
        self.zero(a);
        self.inc_by(a, b);
    }

    /// If a variable is equal to another variable, execute the code
    fn if_equal(&mut self, a: &Variable, b: &Variable) {
        // TODO Too long code for such a simple operation
        // Set flag temp2 to 1
        self.set(&"2".to_string(), &1);

        self.sub(a, b);
        self.goto(a);
        self.code += "[";
        self.set(&"2".to_string(), &0);
        self.add(a, b);
        self.goto(a);
        self.code += "]";

        // Check execution flag
        self.goto(&"2".to_string());
        self.code += "[";
        self.add(a, b);

        self.block_stack.push(BlockStack::IfEqual {
            a: a.clone(),
            b: b.clone(),
        });
    }

    /// If a variable is not equal to another variable, execute the code
    fn if_not_equal(&mut self, a: &Variable, b: &Variable) {
        self.sub(a, b);
        self.goto(a);
        self.code += "[";
        self.add(a, b);

        self.block_stack.push(BlockStack::IfNotEqual {
            a: a.clone(),
            b: b.clone(),
        });
    }

    /// Until a variable is equal to another variable, execute the code
    fn until_equal(&mut self, a: &Variable, b: &Variable) {
        self.sub(a, b);
        self.goto(a);
        self.code += "[";
        self.add(a, b);

        self.block_stack.push(BlockStack::UntilEqual {
            a: a.clone(),
            b: b.clone(),
        })
    }

    /// While a variable is not zero, execute the code
    fn while_not_zero(&mut self, a: &Variable) {
        self.goto(a);
        self.code += "[";
        self.block_stack.push(BlockStack::WhileNotZero());
    }

    /// End blocks
    fn end(&mut self) {
        let b = self.block_stack.pop().unwrap();
        match b {
            BlockStack::WhileNotZero() => {
                self.code += "]";
            }
            BlockStack::IfNotEqual { a, b } => {
                self.sub(&a, &b);
                self.goto(&"0".to_string());
                self.code += "]";
                self.add(&a, &b);
            }
            BlockStack::UntilEqual { a, b } => {
                self.sub(&a, &b);
                self.goto(&a);
                self.code += "]";
                self.add(&a, &b);
            }
            BlockStack::IfEqual { a, b } => {
                self.sub(&a, &b);
                self.zero(&"2".to_string());
                self.code += "]";
                self.add(&a, &b);
            }
        }
    }

    /// Copy variable `from` to `to`
    fn copy(&mut self, from: &Variable, to: &Variable) {
        self.zero(to);

        self.goto(from);

        // Move `from` to temp0 and temp1
        self.code += "[-"; // TODO Use self. methods
        self.goto(&"0".to_string());
        self.code += "+>+";
        self.ptr += 1;
        self.goto(from);
        self.code += "]";

        // Move `temp0` to `from`
        self.goto(&"0".to_string());
        self.code += "[-"; // TODO Use self. methods
        self.goto(from);
        self.code += "+";
        self.goto(&"0".to_string());
        self.code += "]";

        // Move `temp1` to `to`
        self.goto(&"1".to_string());
        self.code += "[-"; // TODO Use self. methods
        self.goto(to);
        self.code += "+";
        self.goto(&"1".to_string());
        self.code += "]";

        // Temp0 and temp1 are zeroed automatically
        self.goto(to);
    }

    // Might implement as an instruction one day
    // // Helper: Move value from one cell to another
    // fn move_value(&mut self, from: &Variable, to: &Variable) {
    //     self.zero(to);
    //     self.goto(from);
    //     self.code += "[-";
    //     self.goto(to);
    //     self.code += "+";
    //     self.goto(from);
    //     self.code += "]";
    // }

    /// Add variable `from` to variable `to`
    fn add(&mut self, to: &Variable, from: &Variable) {
        self.goto(from);

        // Move `from` to temp0 and temp1
        self.code += "[-"; // TODO Use self. methods
        self.goto(&"0".to_string());
        self.code += "+>+";
        self.ptr += 1;
        self.goto(from);
        self.code += "]";

        // Move `temp0` to `from`
        self.goto(&"0".to_string());
        self.code += "[-"; // TODO Use self. methods
        self.goto(from);
        self.code += "+";
        self.goto(&"0".to_string());
        self.code += "]";

        // Move `temp1` to `to`
        self.goto(&"1".to_string());
        self.code += "[-"; // TODO Use self. methods
        self.goto(to);
        self.code += "+";
        self.goto(&"1".to_string());
        self.code += "]";

        // Temp0 and temp1 are zeroed automatically
        self.goto(to);
    }

    /// Subtract variable `from` from variable `to`
    fn sub(&mut self, to: &Variable, from: &Variable) {
        self.goto(from);

        // Move `from` to temp0 and temp1
        self.code += "[-"; // TODO Use self. methods
        self.goto(&"0".to_string());
        self.code += "+>+";
        self.ptr += 1;
        self.goto(from);
        self.code += "]";

        // Move `temp0` to `from`
        self.goto(&"0".to_string());
        self.code += "[-"; // TODO Use self. methods
        self.goto(from);
        self.code += "+";
        self.goto(&"0".to_string());
        self.code += "]";

        // Move `temp1` to `to`
        self.goto(&"1".to_string());
        self.code += "[-"; // TODO Use self. methods
        self.goto(to);
        self.code += "-";
        self.goto(&"1".to_string());
        self.code += "]";

        // Temp0 and temp1 are zeroed automatically
        self.goto(to);
    }

    fn read(&mut self, a: &Variable) {
        self.goto(a);
        self.zero(a); // Depends on implementation, but the debugger I'm using needs this
        // Also should check if it has been accessed before
        self.code += ",";
    }

    fn print(&mut self, a: &Variable) {
        self.goto(a);
        self.code += ".";
    }

    /// Increment a variable by number
    fn inc_by(&mut self, a: &Variable, b: &Immediate) {
        self.goto(a);
        self.code += &*"+".repeat(*b as usize);
    }

    /// Decrement a variable by number
    fn dec_by(&mut self, a: &Variable, b: &Immediate) {
        self.goto(a);
        self.code += &*"-".repeat(*b as usize);
    }

    /// Zero out a variable
    fn zero(&mut self, a: &Variable) {
        // TODO Fuck off if the variable has not been accessed yet

        self.goto(a);
        self.code += "[-]";
    }

    /// Move pointer to a variable
    fn goto(&mut self, a: &Variable) {
        self.move_by(self.parsed.variables.get(a).unwrap() - self.ptr)
    }

    /// Move pointer by `diff`
    fn move_by(&mut self, diff: i32) {
        self.ptr += diff;
        if diff < 0 {
            self.code += &*"<".repeat(diff.abs() as usize);
        }
        if diff > 0 {
            self.code += &*">".repeat(diff.abs() as usize);
        }
    }
}
