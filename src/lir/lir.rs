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

    // Data manipulation
    Binary {
        op: BinaryOp,
        modified: Location,
        consumed: Value,
    },

    // Variable modification
    Copy {
        from: Value,
        to: Location,
    },

    // I/O
    Read(Location),
    Print(Value),

    // Control flow
    Match(Location),
    Case(Immediate),
    EndMatch,
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
            Instruction::Push(n) => format!("Push({})", n),
            Instruction::Pop => "pop".to_string(),
            Instruction::Dup => "Dup".to_string(),
            Instruction::Binary {
                op,
                modified,
                consumed,
            } => format!("{:?}", op),
            Instruction::Copy { from, to } => format!("Copy {:?} to {:?}", from, to),
            Instruction::Read(loc) => format!("Read {:?}", loc),
            Instruction::Print(val) => format!("Print {:?}", val),
            Instruction::Match(loc) => format!("Match {:?}", loc),
            Instruction::Case(n) => format!("Case {}", n),
            Instruction::EndMatch => "EndMatch".to_string(),
        }
    }
}
