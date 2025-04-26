use crate::lir::instructions::InstructionsParsed;
use crate::lir::lir::{Immediate, Instruction, Variable};
use anyhow::Result;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct Codegen {
    code: String,
    ptr: i32,
    pub instructions: Vec<Instruction>,
    parsed: InstructionsParsed,
}

impl Codegen {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            code: String::new(),
            ptr: 0,
            instructions,
            parsed: InstructionsParsed::default(),
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
            Instruction::Copy { a, b } => self.copy(&a, &b),
            Instruction::Inc(a) => self.inc_by(&a, &1),
            Instruction::Dec(a) => self.dec_by(&a, &1),
            Instruction::IncBy(a, b) => self.inc_by(&a, &b),
            Instruction::DecBy(a, b) => self.dec_by(&a, &b),
            Instruction::Set(a, b) => self.set(&a, &b),
            Instruction::Read(a) => self.read(&a),
            Instruction::Print(a) => self.print(&a),
            Instruction::Add { a, b } => self.add(&a, &b),
            Instruction::Sub { a, b } => self.sub(&a, &b),
            Instruction::Raw(raw) => self.code += &*raw,
        }

        Ok(())
    }

    /// Set a variable to a value
    fn set(&mut self, a: &Variable, b: &Immediate) {
        self.zero(a);
        self.inc_by(a, b);
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
