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
    EndCase,
    CaseDefault,
    EndCaseDefault,
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
            Instruction::Swap => "Swap".to_string(),
            Instruction::Binary {
                op,
                modified,
                consumed,
            } => format!("{:?} {:?} {:?}", op, modified, consumed),
            Instruction::Copy { from, to } => format!("Copy {:?} to {:?}", from, to),
            Instruction::Read(loc) => format!("Read {:?}", loc),
            Instruction::Print(val) => format!("Print {:?}", val),
            Instruction::Match(loc) => format!("Match {:?}", loc),
            Instruction::Case(n) => format!("Case {}", n),
            Instruction::CaseDefault => "CaseDefault".to_string(),
            Instruction::EndMatch => "EndMatch".to_string(),
            Instruction::EndCase => "EndCase".to_string(),
            Instruction::EndCaseDefault => "EndCaseDefault".to_string(),
        }
    }
}
