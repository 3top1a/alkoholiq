use std::io::{Read, Write};

const MAX_INSTRUCTIONS: usize = 10_000_000;

pub struct Interpreter {
    tape: [u8; 30000],
    pointer: i32,
    instructions_ran: usize,
}

// This interpreter is slow as fuck but will do
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            tape: [0; 30000],
            pointer: 0,
            instructions_ran: 0,
        }
    }

    pub fn run(mut self, code: &str, input: &mut impl Read, output: &mut impl Write) {
        let jump_table = Self::calculate_jumps(code);
        let mut instruction_index = 0;
        let code: Vec<char> = code.chars().collect();

        while instruction_index < code.len() {
            self.instructions_ran += 1;
            if self.instructions_ran == MAX_INSTRUCTIONS {
                panic!("Too many instructions");
            }

            match code[instruction_index] {
                '>' => self.pointer_right(),
                '<' => self.pointer_left(),
                '+' => {
                    self.tape[self.pointer as usize] =
                        self.tape[self.pointer as usize].wrapping_add(1)
                }
                '-' => {
                    self.tape[self.pointer as usize] =
                        self.tape[self.pointer as usize].wrapping_sub(1)
                }
                '.' => {
                    output
                        .write_all(&[self.tape[self.pointer as usize]])
                        .unwrap();
                }
                ',' => {
                    let mut buf = [0; 1];
                    let read = input.read(&mut buf);

                    if let Ok(0) = read {
                        self.tape[self.pointer as usize] = 0;
                        return;
                    }

                    if read.unwrap() > 0 {
                        self.tape[self.pointer as usize] = buf[0];
                    }
                }
                '[' => {
                    if self.tape[self.pointer as usize] == 0 {
                        instruction_index = jump_table[instruction_index];
                    }
                }
                ']' => {
                    if self.tape[self.pointer as usize] != 0 {
                        instruction_index = jump_table[instruction_index];
                    }
                }
                '%' => {
                    // Check all temporary variables are zero
                    assert!(self.tape.iter().rev().take(20).all(|&x| x == 0));
                }
                _ => {}
            }

            instruction_index += 1;
        }
    }

    fn calculate_jumps(code: &str) -> Vec<usize> {
        let mut jump_table = vec![0; code.len()];
        let mut stack = Vec::new();

        for (i, c) in code.chars().enumerate() {
            match c {
                '[' => stack.push(i),
                ']' => {
                    if let Some(start) = stack.pop() {
                        jump_table[start] = i;
                        jump_table[i] = start;
                    }
                }
                _ => {}
            }
        }

        jump_table
    }

    fn pointer_left(&mut self) {
        self.pointer -= 1;
        if self.pointer < 0 {
            self.pointer += self.tape.len() as i32;
        }
    }

    fn pointer_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.tape.len() as i32 {
            self.pointer -= self.tape.len() as i32;
        }
    }
}
