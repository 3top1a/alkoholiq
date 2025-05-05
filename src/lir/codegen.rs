use crate::lir::analysis::InstructionsAnalysis;
use crate::lir::instruction::{Immediate, Instruction, Instruction::*, Variable};
use anyhow::Result;
use std::string::ToString;

#[derive(Debug, Clone)]
enum BlockStack {
    IfEqual { a: Variable, b: Variable },
    IfEqualConst { a: Variable },
    IfNotEqual { a: Variable, b: Variable },
    UntilEqual { a: Variable, b: Variable },
    WhileNotZero(Variable),
    IfNotEqualConst { a: Variable, b: Immediate },
}

#[derive(Debug, Clone)]
pub struct Codegen {
    code: String,
    ptr: i32,
    pub instructions: Vec<Instruction>,
    parsed: InstructionsAnalysis,
    block_stack: Vec<BlockStack>,
    instruction_separator: String,
}

impl Codegen {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            code: String::new(),
            ptr: 0,
            instructions,
            parsed: InstructionsAnalysis::default(),
            block_stack: Vec::new(),
            instruction_separator: String::new(),
        }
    }

    pub fn new_test(instructions: Vec<Instruction>) -> Self {
        // Self::new but with the instruction separator set to `#`
        Self {
            code: String::new(),
            ptr: 0,
            instructions,
            parsed: InstructionsAnalysis::default(),
            block_stack: Vec::new(),
            instruction_separator: String::from("#"),
        }
    }

    pub fn codegen(mut self) -> Result<String> {
        self.parsed = InstructionsAnalysis::new(self.instructions.clone())?;

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
            PrintC(a) => self.printc(&a),
            Add { a, b } => self.add(&a, &b),
            Sub { a, b } => self.sub(&a, &b),
            Raw(raw) => self.code += &*raw,
            IfEqual { a, b } => self.if_equal(&a, &b),
            IfEqualConst { a, b } => self.if_equal_const(&a, &b),
            IfNotEqual { a, b } => self.if_not_equal(&a, &b),
            IfNotEqualConst { a, b } => self.if_not_equal_const(&a, &b),
            UntilEqual { a, b } => self.until_equal(&a, &b),
            WhileNotZero(a) => self.while_not_zero(&a),
            End => self.end(),
            Compare { a, b, res } => self.compare(&a, &b, &res),
            PrintMsg(msg) => self.print_msg(msg),
            Mul { a, b } => self.mul(&a, &b),
            Div {
                a,
                b,
                remainder: r,
                quotient: q,
            } => self.div(&a, &b, &r, &q),
            Push(a) => self.stack_push(&a),
            Pop(a) => self.stack_pop(&a),
        }

        self.code += &self.instruction_separator;

        Ok(())
    }

    /// Set a variable to a value
    fn set(&mut self, a: &Variable, b: &Immediate) {
        self.zero(a);
        self.inc_by(a, b);
    }

    /// Divide two variables
    ///
    /// Uses temporary variables `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8` and `9`
    fn div(&mut self, a: &Variable, b: &Variable, remainder: &Variable, quotient: &Variable) {
        // Algo: Sub `a` by `b` until `a` is less than `b`

        // TODO use less temp variables

        // Save for later
        self.copy(a, &"9".to_string());
        self.copy(b, &"8".to_string());
        self.zero(remainder);
        self.zero(quotient);

        // While flag
        self.set(&"7".to_string(), &1);
        self.while_not_zero(&"7".to_string());

        self.compare(a, b, &"6".to_string());

        // If `a` is more than `b`, subtract `b` from `a`
        self.if_not_equal_const(&"6".to_string(), &1);
        self.sub(a, b);
        self.inc_by(quotient, &1);
        self.end();

        // If `a` is less than `b`, set flag to 0
        self.if_not_equal_const(&"6".to_string(), &2);
        self.set(&"7".to_string(), &0);
        self.end();

        self.end();

        self.zero(&"6".to_string());
        self.move_value(a, remainder);
        self.move_value(&"9".to_string(), a);
        self.move_value(&"8".to_string(), b);
        self.goto(quotient);
    }

    /// Multiply two variables
    ///
    /// Uses temporary variables `0`, `1`, `2` and `3`
    fn mul(&mut self, a: &Variable, b: &Variable) {
        // Algo: Add `a` to itself `b` times
        self.copy(b, &"2".to_string());

        self.while_not_zero(&"2".to_string());

        self.dec_by(&"2".to_string(), &1);
        self.add(&"3".to_string(), a);

        self.end();

        self.move_value(&"3".to_string(), a);
    }

    /// Compare two variables and store the result in a third variable
    ///
    /// Uses temporary variables `3`, `4` and `5`
    fn compare(&mut self, a: &Variable, b: &Variable, res: &Variable) {
        self.zero(res);

        // TODO probably could be optimized

        self.if_not_equal(a, b);

        self.set(res, &8);
        self.set(&"4".to_string(), &1);
        self.set(&"5".to_string(), &0);

        self.while_not_zero(&"4".to_string());

        self.dec_by(a, &1);
        self.dec_by(b, &1);
        self.inc_by(&"5".to_string(), &1);

        self.if_equal_const(a, &0);
        self.set(res, &1);
        self.set(&"4".to_string(), &0);
        self.end();

        self.if_equal_const(b, &0);
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
    }

    /// If a variable is equal to a constant, execute the code
    ///
    ///  Uses temporary variable `1`, `2` and `3`
    fn if_equal_const(&mut self, a: &Variable, b: &Immediate) {
        debug_assert_ne!(a, &"1".to_string());
        debug_assert_ne!(a, &"2".to_string());
        debug_assert_ne!(a, &"3".to_string());

        // TODO Too long code for such a common operation

        // Set flag temp2 to 1
        self.set(&"2".to_string(), &1);
        self.zero(&"3".to_string());

        // Subtract `b` from `a`
        self.dec_by(a, b);

        // If they're equal, `a` will be zero, and the following will not be run
        self.goto(a);
        self.code += "[";
        self.set(&"2".to_string(), &0);
        self.move_value(a, &"3".to_string());
        self.goto(a);
        self.code += "]";

        // Move from `temp3` to `a`; this could be an Add and Zero but that's too long
        self.while_not_zero(&"3".to_string());
        self.dec_by(&"3".to_string(), &1);
        self.inc_by(a, &1);
        self.end();

        self.inc_by(a, b); // Preserve

        // Check execution flag
        self.goto(&"2".to_string());
        self.code += "[";
        self.zero(&"2".to_string());

        self.block_stack
            .push(BlockStack::IfEqualConst { a: a.clone() });
    }

    /// If a variable is not equal to a constant, execute the code
    fn if_not_equal_const(&mut self, a: &Variable, b: &Immediate) {
        // Subtract `b` from `a`
        self.dec_by(a, b);
        self.copy(a, &"2".to_string());
        self.inc_by(a, b);

        // If they're not equal, `a` will be zero, and the following will be run
        self.goto(&"2".to_string());
        self.code += "[";
        self.zero(&"2".to_string());

        self.block_stack.push(BlockStack::IfNotEqualConst {
            a: a.clone(),
            b: *b,
        });
    }

    /// If a variable is equal to another variable, execute the code
    ///
    /// Uses temporary variable `1`, `2` and `3`
    fn if_equal(&mut self, a: &Variable, b: &Variable) {
        debug_assert_ne!(a, b);
        debug_assert_ne!(a, &"1".to_string());
        debug_assert_ne!(a, &"2".to_string());
        debug_assert_ne!(a, &"3".to_string());
        debug_assert_ne!(b, &"1".to_string());
        debug_assert_ne!(b, &"2".to_string());
        debug_assert_ne!(b, &"3".to_string());

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
        self.zero(&"2".to_string());

        self.block_stack.push(BlockStack::IfEqual {
            a: a.clone(),
            b: b.clone(),
        });
    }

    /// If a variable is not equal to another variable, execute the code
    ///
    /// Uses temporary variable `1`, `2` and `3`
    fn if_not_equal(&mut self, a: &Variable, b: &Variable) {
        debug_assert_ne!(a, b);
        debug_assert_ne!(a, &"1".to_string());
        debug_assert_ne!(a, &"2".to_string());
        debug_assert_ne!(a, &"3".to_string());
        debug_assert_ne!(b, &"1".to_string());
        debug_assert_ne!(b, &"2".to_string());
        debug_assert_ne!(b, &"3".to_string());

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
        debug_assert_ne!(a, b);

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
            BlockStack::IfEqual { .. } => {
                self.zero(&"2".to_string());
                self.goto(&"2".to_string());
                self.code += "]";
            }
            BlockStack::IfEqualConst { .. } => {
                self.zero(&"2".to_string());
                self.code += "]";
            }
            BlockStack::IfNotEqualConst { a, b } => {
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

    /// Pretty print a number
    fn printc(&mut self, a: &Variable) {
        self.set(&"15".to_string(), &100);
        self.div(a, &"15".to_string(), &"10".to_string(), &"11".to_string());

        self.set(&"15".to_string(), &10);
        self.div(
            &"10".to_string(),
            &"15".to_string(),
            &"12".to_string(),
            &"14".to_string(),
        );

        // So temp11 is hundreds, temp14 is tens, temp12 is ones
        self.set(&"15".to_string(), &0);

        self.if_not_equal(&"11".to_string(), &"15".to_string());
        self.inc_by(&"11".to_string(), &48);
        self.print(&"11".to_string());
        self.end();

        self.if_not_equal(&"14".to_string(), &"15".to_string());
        self.inc_by(&"14".to_string(), &48);
        self.print(&"14".to_string());
        self.end();

        self.if_not_equal(&"12".to_string(), &"15".to_string());
        self.inc_by(&"12".to_string(), &48);
        self.print(&"12".to_string());
        self.end();

        // Zero out temp variables
        self.zero(&"10".to_string());
        self.zero(&"11".to_string());
        self.zero(&"12".to_string());
        self.zero(&"13".to_string());
        self.zero(&"14".to_string());
        self.zero(&"15".to_string());
    }

    fn print_msg(&mut self, msg: String) {
        let msg = msg
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r");

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
        self.code += "[-]";
    }

    /// Push on stack
    fn stack_push(&mut self, a: &Variable) {
        let ID = 1; // TODO let user decide ID

        self.copy(a, &"2".to_string()); // TODO Make a `copy` but for internal purposes (Does not clear one temp)

        self.goto(&"2".to_string());

        self.while_not_zero(&"2".to_string());

        self.code += "-";

        self.goto_end_of_vars();
        self.code += ">>";

        self.code += "[>>]";

        self.code += ">+<";

        self.code += "<<";

        self.code += "[<<]";

        self.goto(&"2".to_string());

        self.end();

        self.goto_end_of_vars();
        self.code += ">>";
        self.code += "[>>]";
        self.code += &*"+".repeat(ID as usize);
        self.code += "<<";
        self.code += "[<<]";

        self.goto(a);
    }

    /// Pop from the stack
    fn stack_pop(&mut self, a: &Variable) {
        self.zero(a);

        // Goto the last stack position
        self.goto_end_of_vars();
        self.code += ">>";
        self.code += "[>>]<";

        self.code += "[";

        // Dec and move it to a
        self.code += "-";
        self.code += "<";
        self.code += "[<<]";

        self.goto(a);
        self.code += "+";

        // Goto the last stack position again
        self.goto_end_of_vars();
        self.code += ">>";
        self.code += "[>>]<";

        self.code += "]";

        // Remove the flag
        self.code += "<#[-]";

        self.code += "<<[<<]";

        self.goto(a);
    }

    /// Move pointer to end of variables
    fn goto_end_of_vars(&mut self) {
        self.move_by(self.parsed.variable_count - self.ptr);
    }

    /// Move pointer to a variable
    fn goto(&mut self, a: &Variable) {
        self.move_by(
            self.parsed
                .variables
                .get(a)
                .unwrap_or_else(|| panic!("Unable to retrieve position of variable {a}"))
                - self.ptr,
        )
    }

    /// Move pointer by `diff`
    fn move_by(&mut self, diff: i32) {
        self.ptr += diff;
        if diff < 0 {
            self.code += &*"<".repeat(diff.unsigned_abs() as usize);
        }
        if diff > 0 {
            self.code += &*">".repeat(diff.unsigned_abs() as usize);
        }
    }
}
