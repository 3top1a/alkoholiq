use crate::lir::lir::LIR;

/// This is a special constant used as a placeholder for the input operation.
/// Corresponds to ASCII value of 'E'
pub const INPUT: u8 = 69;

#[derive(Clone, Debug, PartialEq)]
enum MemorySpace {
    Byte,
    IfPadding,
}

impl MemorySpace {
    pub fn size(&self) -> usize {
        match self {
            MemorySpace::Byte => 1,
            MemorySpace::IfPadding => 2,
        }
    }
}

/// Lir code generator to brainfuck.
///
/// It maintains a stack itself for checking against the interpreted stack.
/// This also means that all operations must be valid (e.g. no Dup on an empty stack)
/// as it would panic otherwise.
pub struct Codegen {
    pub stack: Vec<u8>,
    pub code: String,
    pub memory: Vec<MemorySpace>,
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            stack: vec![],
            code: String::new(),
            memory: vec![],
        }
    }

    fn memory_length(&self) -> usize {
        self.memory
            .iter()
            .map(|x| x.size())
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn memory_index_of_index(&self, target: usize) -> usize {
        // Target is x Memory::Bytes from the top
        let mut index = 0;
        for mem in self.memory.iter() {
            if mem == &MemorySpace::Byte {
                index += 1;
            }
            if index
                == self
                    .memory
                    .iter()
                    .filter(|&x| x == &MemorySpace::Byte)
                    .count()
                    - target
            {
                break;
            }
        }
        index
    }

    pub fn generate(&mut self, input: Vec<LIR>) {
        for lir in input {
            self.code += format!("{:?} ", lir).as_str();
            match lir {
                LIR::Push(n) => self.push(n),
                LIR::Pop => self.pop(),
                LIR::False => self.push(0),
                LIR::True => self.push(1),
                LIR::Add => self.add(),
                LIR::Sub => self.sub(),
                LIR::Print => self.print(),
                LIR::Var(ptr) => self.var(ptr),
                LIR::Dup => self.dup(),
                LIR::Eq => self.eq(),
                LIR::Input => self.input(),
                LIR::StartIf => self.start_if(),
                LIR::StartElse => self.start_else(),
                LIR::EndIf => self.end_if(),
            }
            self.code += "\n";
        }
    }

    fn push(&mut self, n: u8) {
        self.stack.push(n);
        self.memory.push(MemorySpace::Byte);
        self.code += ">";
        self.code += "+".repeat(n as usize).as_str();
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.memory.pop();
        self.code += "[-]<";
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.memory.pop();
        self.memory.pop();
        self.stack.push(a + b);
        self.code += "[-<+>]<";
        self.memory.push(MemorySpace::Byte);
    }

    fn sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.memory.pop();
        self.memory.pop();
        self.stack.push(b.wrapping_sub(a));
        self.code += "[-<->]<";
        self.memory.push(MemorySpace::Byte);
    }

    fn print(&mut self) {
        let _n = self.stack.pop().unwrap();
        self.memory.pop();
        // use std::str::from_utf8;
        // print!("{}", from_utf8(&[_n]).unwrap());

        self.code += ".";
        self.code += "[-]<";
    }

    fn input(&mut self) {
        self.stack.push(INPUT);
        self.memory.push(MemorySpace::Byte);

        self.code += ">,";
    }

    fn var(&mut self, n: usize) {
        let value = self.stack[n];
        self.stack.push(value);
        self.memory.push(MemorySpace::Byte);

        let diff = self.memory_index_of_index(n);

        // E.g. Given a stack of [2; 1] it should generate
        // <
        // [->>+>+<<<]
        // >>>
        // [-<<<+>>>]<
        // The end result should be [2; 1; 2]

        // Go to source
        self.code += "<".repeat(diff).as_str();

        // Loop; take one
        self.code += "[-";
        // Go do temp space
        self.code += ">".repeat(diff + 1).as_str();
        // Push two of the same values
        self.code += "+>+<";
        // Go back for next iter
        self.code += "<".repeat(diff + 1).as_str();
        self.code += "]";
        self.code += ">".repeat(diff + 1).as_str(); // Go back to top

        // Now move the temp extra back to the source
        self.code += ">[-";
        self.code += "<".repeat(diff + 2).as_str();
        self.code += "+";
        self.code += ">".repeat(diff + 2).as_str();
        self.code += "]<";
    }

    fn dup(&mut self) {
        let n = self.stack.pop().unwrap();
        self.stack.push(n);
        self.stack.push(n);

        self.code += "[->+>+<<]>>[-<<+>>]<"
    }

    fn eq(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(if a == b { 1 } else { 0 });

        // This should be optimized, it's quite long
        /*
        >++>+++++
        calc the diff
        [-<->]
        set a flag
        +<
        clear the flag if the diff is not zero
        [->[-]<]
        move result one left
        >
        [-<+>]
        <
         */

        self.code += "[-<->]+<[->[-]<]>[-<+>]<"
    }

    fn start_if(&mut self) {
        // Actually due to codegen it needs to act as if it's always true
        self.code += ">+<[>->";
    }

    fn start_else(&mut self) {
        // TODO Start else is mandatory
        self.code += "<<[-]]>[-";
    }

    fn end_if(&mut self) {
        self.code += "]>";
    }
}
