use crate::lir::instructions::InstructionsParsed;
use crate::lir::lir::{Immediate, Instruction, Instruction::*, Variable};
use anyhow::Result;
use std::string::ToString;

#[derive(Debug, Clone)]
enum BlockStack {
    IfEqual { a: Variable, b: Variable },
    IfEqualConst { a: Variable },
    IfNotEqual { a: Variable, b: Variable },
    UntilEqual { a: Variable, b: Variable },
    WhileNotZero(Variable),
}

#[derive(Debug, Clone)]
pub struct Codegen {
    code: String,
    ptr: i32,
    pub instructions: Vec<Instruction>,
    parsed: InstructionsParsed,
    block_stack: Vec<BlockStack>,
    accessed_variables: Vec<Variable>,
}

impl Codegen {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            code: String::new(),
            ptr: 0,
            instructions,
            parsed: InstructionsParsed::default(),
            block_stack: Vec::new(),
            accessed_variables: Vec::new(),
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
            IfEqualConst { a, b } => self.if_equal_const(&a, &b),
            IfNotEqual { a, b } => self.if_not_equal(&a, &b),
            UntilEqual { a, b } => self.until_equal(&a, &b),
            WhileNotZero(a) => self.while_not_zero(&a),
            End => self.end(),
            Compare { a, b, res } => self.compare(&a, &b, &res),
            PrintMsg(msg) => self.print_msg(msg),
        }

        Ok(())
    }

    /// Set a variable to a value
    fn set(&mut self, a: &Variable, b: &Immediate) {
        self.zero(a);
        self.inc_by(a, b);
    }

    /// Compare two variables and store the result in a third variable
    fn compare(&mut self, a: &Variable, b: &Variable, res: &Variable) {
        self.zero(res);
        // self.if_equal(a, b); // Does nothing
        // self.end();

        // TODO probably could be optimized

        self.if_not_equal(a, b);

        self.set(res, &8);
        self.set(&"3".to_string(), &0);
        self.set(&"4".to_string(), &1);
        self.set(&"5".to_string(), &0);

        self.while_not_zero(&"4".to_string());

        self.dec_by(a, &1);
        self.dec_by(b, &1);
        self.inc_by(&"5".to_string(), &1);

        self.if_equal(a, &"3".to_string()); // FIXME Would be cleaner if `if` supported immediate
        self.set(res, &1);
        self.set(&"4".to_string(), &0);
        self.end();

        self.if_equal(b, &"3".to_string());
        self.set(res, &2);
        self.set(&"4".to_string(), &0);
        self.end();

        self.end();

        // Add numbers back up to original
        self.while_not_zero(&"5".to_string());
        self.inc_by(a, &1);
        self.inc_by(b, &1);
        self.dec_by(&"5".to_string(), &1);
        self.end();

        self.end();

        // Should be zeroed automatically
    }

    /// If a variable is equal to a constant, execute the code
    fn if_equal_const(&mut self, a: &Variable, b: &Immediate) {
        debug_assert_ne!(a, &"1".to_string());
        debug_assert_ne!(a, &"2".to_string());

        // TODO Too long code for such a common operation

        // Set flag temp2 to 1
        self.set(&"2".to_string(), &1);

        self.dec_by(a, b);
        self.copy(a, &"3".to_string());
        self.goto(&"3".to_string());
        self.code += "[";
        self.set(&"2".to_string(), &0);
        self.goto(&"3".to_string());
        self.zero(&"3".to_string());
        self.code += "]";
        self.inc_by(a, b);

        // Check execution flag
        self.goto(&"2".to_string());
        self.code += "[";

        self.block_stack
            .push(BlockStack::IfEqualConst { a: a.clone() });
    }

    /// If a variable is equal to another variable, execute the code
    ///
    /// Uses temporary variable `1`, `2` and `3`
    fn if_equal(&mut self, a: &Variable, b: &Variable) {
        debug_assert_ne!(a, b);
        debug_assert_ne!(a, &"1".to_string());
        debug_assert_ne!(a, &"2".to_string());
        debug_assert_ne!(b, &"1".to_string());
        debug_assert_ne!(b, &"2".to_string());

        // TODO Too long code for such a common operation
        // Set flag temp2 to 1
        self.set(&"2".to_string(), &1);

        self.sub(a, b);
        self.copy(a, &"3".to_string());
        self.goto(&"3".to_string());
        self.code += "[";
        self.set(&"2".to_string(), &0);
        self.goto(&"3".to_string());
        self.zero(&"3".to_string());
        self.code += "]";
        self.add(a, b);

        // Check execution flag
        self.goto(&"2".to_string());
        self.code += "[";

        self.block_stack.push(BlockStack::IfEqual {
            a: a.clone(),
            b: b.clone(),
        });
    }

    /// If a variable is not equal to another variable, execute the code
    ///
    /// Does not use any temporary variables
    fn if_not_equal(&mut self, a: &Variable, b: &Variable) {
        self.sub(a, b);
        self.goto(a);
        self.copy(a, &"2".to_string());
        self.goto(&"2".to_string());
        self.code += "[";
        self.zero(&"2".to_string());
        self.add(a, b);

        self.block_stack.push(BlockStack::IfNotEqual {
            a: a.clone(),
            b: b.clone(),
        });
    }

    /// Until a variable is equal to another variable, execute the code
    ///
    /// Does not use any temporary variables
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
    ///
    /// Does not use any temporary variables
    fn while_not_zero(&mut self, a: &Variable) {
        self.goto(a);
        self.code += "[";
        self.block_stack.push(BlockStack::WhileNotZero(a.clone()));
    }

    /// End blocks
    fn end(&mut self) {
        let b = self.block_stack.pop().unwrap();
        match b {
            BlockStack::WhileNotZero(a) => {
                self.goto(&a);
                self.code += "]";
            }
            BlockStack::IfNotEqual { a, b } => {
                self.sub(&a, &b);
                self.goto(&"2".to_string());
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
                self.zero(&"2".to_string());
                self.goto(&"2".to_string());
                self.code += "]";
                self.zero(&"2".to_string());
            }
            BlockStack::IfEqualConst { a } => {
                self.zero(&"2".to_string());
                self.goto(&"2".to_string());
                self.code += "]";
                self.zero(&"2".to_string());
            }
        }
    }

    /// Copy variable `from` to `to`
    ///
    /// Uses temporary variables `0` and `1` to store the value
    fn copy(&mut self, from: &Variable, to: &Variable) {
        debug_assert_ne!(from, to);
        debug_assert_ne!(to, &"0".to_string());
        debug_assert_ne!(to, &"1".to_string());

        self.zero(to);
        self.goto(from);

        // Move `from` to temp0 and temp1
        self.code += "[-"; // TODO Use self. methods
        self.goto(&"0".to_string());
        self.code += "+";
        self.goto(&"1".to_string());
        self.code += "+";
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

    // Helper: Move value from one cell to another
    fn move_value(&mut self, from: &Variable, to: &Variable) {
        self.zero(to);
        self.goto(from);
        self.code += "[-";
        self.goto(to);
        self.code += "+";
        self.goto(from);
        self.code += "]";
    }

    /// Add variable `from` to variable `to`
    ///
    /// Uses temporary variables `0` and `1` to store the value
    fn add(&mut self, to: &Variable, from: &Variable) {
        debug_assert_ne!(from, to);
        debug_assert_ne!(to, &"0".to_string());
        debug_assert_ne!(to, &"1".to_string());

        self.goto(from);

        // Move `from` to temp0 and temp1
        self.code += "[-"; // TODO Use self. methods
        self.goto(&"0".to_string());
        self.code += "+";
        self.goto(&"1".to_string());
        self.code += "+";
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
    ///
    /// Uses temporary variables `0` and `1` to store the value
    fn sub(&mut self, to: &Variable, from: &Variable) {
        debug_assert_ne!(from, to);
        debug_assert_ne!(to, &"0".to_string());
        debug_assert_ne!(to, &"1".to_string());

        self.goto(from);

        // Move `from` to temp0 and temp1
        self.code += "[-"; // TODO Use self. methods
        self.goto(&"0".to_string());
        self.code += "+";
        self.goto(&"1".to_string());
        self.code += "+";
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

    fn print_msg(&mut self, msg: String) {
        let mut last = 0;
        self.zero(&"0".to_string());
        for c in msg.chars() {
            let diff = c as i32 - last;
            if diff > 0 {
                self.inc_by(&"0".to_string(), &(diff as u8));
            } else {
                self.dec_by(&"0".to_string(), &(-diff as u8));
            }

            self.code += ".";
            last = c as i32;
        }
        self.zero(&"0".to_string());
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
        self.goto(a);
        if self.accessed_variables.contains(a) {
            self.code += "[-]";
        }
        self.variable_written(a);
    }

    /// Move pointer to a variable
    fn goto(&mut self, a: &Variable) {
        self.move_by(
            self.parsed
                .variables
                .get(a)
                .expect(&format!("Unable to retrieve position of variable {a}"))
                - self.ptr,
        )
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

    /// Helper function to stop adding [-] to variables that have not been used before
    fn variable_written(&mut self, v: &Variable) {
        if self.accessed_variables.contains(v) {
            return;
        }

        self.accessed_variables.push(v.clone());
    }
}
