struct Interpreter {
    code: String,
    memory: [u8; 30000],
    pointer: usize,
}

impl Interpreter {
    pub fn new(code: String) -> Self {
        Self {
            code,
            memory: [0; 30000],
            pointer: 0,
        }
    }

    pub fn interpret(mut self, input: String) -> String {
        let mut output = String::new();
        let mut code = self.code.chars().peekable();
        // Add EOF to input
        let input = input + "\0";
        let mut input = input.chars().peekable();
        while let Some(c) = code.next() {
            match c {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => self.memory[self.pointer] += 1,
                '-' => self.memory[self.pointer] -= 1,
                '.' => output.push(self.memory[self.pointer] as char),
                ',' => self.memory[self.pointer] = input.next().unwrap() as u8,
                '[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            match code.next() {
                                Some('[') => depth += 1,
                                Some(']') => depth -= 1,
                                _ => (),
                            }
                        }
                    }
                }
                ']' => {
                    if self.memory[self.pointer] != 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            match code.next_back() {
                                Some(']') => depth += 1,
                                Some('[') => depth -= 1,
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        output
    }
}
