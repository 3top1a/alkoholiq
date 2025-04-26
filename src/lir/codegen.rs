use crate::lir::instructions::InstructionsParsed;
use crate::lir::lir::{Immediate, Instruction, Variable};
use anyhow::Result;

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
            Instruction::Copy { .. } => todo!(),
            Instruction::Inc(a) => {self.incby(&a, &1)},
            Instruction::Dec(a) => {self.decby(&a, &1)},
            Instruction::Set(a, b) => self.set(&a, &b),
            Instruction::Read(_) => todo!(),
            Instruction::Print(_) => todo!(),
            Instruction::Add { .. } => todo!(),
            Instruction::Sub { .. } => todo!(),
            Instruction::Raw(raw) => self.code += &*raw,
        }

        Ok(())
    }

    fn set(&mut self, a: &Variable, b: &Immediate) {
        self.zero(a);
        self.incby(a, b);
    }

    fn incby(&mut self, a: &Variable, b: &Immediate) {
        self.goto(a);
        self.code += &*"+".repeat(*b as usize);
    }

    fn decby(&mut self, a: &Variable, b: &Immediate) {
        self.goto(a);
        self.code += &*"-".repeat(*b as usize);
    }

    /// Zero out a variable
    fn zero(&mut self, a: &Variable) {
        self.goto(a);
        self.code += "[-]";
    }

    /// Move pointer to a variable
    fn goto(&mut self, a: &Variable) {
        self.moveby(self.parsed.variables.get(a).unwrap() - self.ptr)
    }

    /// Move pointer by `diff`
    fn moveby(&mut self, diff: i32) {
        self.ptr += diff;
        if diff < 0 {
            self.code += &*"<".repeat(diff.abs() as usize);
        }
        if diff > 0 {
            self.code += &*">".repeat(diff.abs() as usize);
        }
    }
}
