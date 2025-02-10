use crate::lir::instructions::Instructions;

type Immediate = u8;

#[derive(Debug, Clone)]
pub enum Location {
    Stack,
    Variable(usize),
}

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(Immediate),
    Location(Location),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack operations
    Push(Immediate),
    Pop,
    Dup,
    Swap,

    // Data manipulation
    Binary {
        op: BinaryOp,
        modified: Location,
        consumed: Value,
    },

    // Variable modification
    Move {
        from: Value,
        to: Location,
    },

    // I/O
    Read(Location),
    Print(Value),

    // Control flow
    Match {
        source: Location,
        cases: Vec<(Immediate, Instructions)>,
        default: Instructions,
    },

    While {
        source: Location,
        body: Instructions,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Instruction {
    pub fn debug(&self) -> String {
        match self {
            Instruction::Push(n) => format!("(Push {})", n),
            Instruction::Pop => "(Pop)".to_string(),
            Instruction::Dup => "(Dup)".to_string(),
            Instruction::Swap => "(Swap)".to_string(),
            Instruction::Binary {
                op,
                modified,
                consumed,
            } => format!("({:?} {:?} {:?})", op, modified, consumed),
            Instruction::Move { from, to } => format!("(Move {:?} {:?})", from, to),
            Instruction::Read(loc) => format!("(Read {:?})", loc),
            Instruction::Print(val) => format!("(Print {:?})", val),
            Instruction::Match { source, .. } => {
                format!("(Match {:?})", source)
            }
            Instruction::While { source, .. } => {
                format!("(While {:?})", source)
            }
        }
    }
}
