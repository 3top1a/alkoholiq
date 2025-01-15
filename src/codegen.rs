use std::collections::HashMap;
use crate::ast::Expression;

pub struct Codegen {
    // Track current memory position
    ptr: usize,
    // Store generated code
    code: String,
    // Track variables and their memory locations
    variables: HashMap<String, usize>,
    // Track next available memory position
    next_mem: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            ptr: 0,
            variables: HashMap::new(),
            next_mem: 0,
            code: String::new(),
        }
    }

    pub fn generate(&mut self, ast: Vec<Expression>) -> String {
        for expr in ast.iter() {
            self.generate_expression(expr);
        }
        self.code.clone()
    }

    fn move_to(&mut self, target: usize) {
        let diff = target as i32 - self.ptr as i32;
        if diff > 0 {
            self.code.push_str(&">".repeat(diff as usize));
        } else if diff < 0 {
            self.code.push_str(&"<".repeat(-diff as usize));
        }
        self.ptr = target;
    }

    fn add_value(&mut self, value: u8) {
        self.code.push_str(&format!("{}", "+".repeat(value as usize)));
    }

    fn sub_value(&mut self, value: u8) {
        self.code.push_str(&format!("{}", "-".repeat(value as usize)));
    }

    fn set_new_value(&mut self, value: u8) {
        // If it's closer to 255, subtract from 255
        if value < 127 {
            self.add_value(value);
        } else {
            self.sub_value((256u16 - value as u16) as u8);
        }
    }

    fn allocate_memory(&mut self, size: usize) -> usize {
        let start = self.next_mem;
        self.next_mem += size;
        start
    }

    fn copy_value(&mut self, from: usize, to: usize) {
        // First need a temporary cell for the copy operation
        let temp = self.next_mem;
        self.next_mem += 1;

        // Move to source position
        self.move_to(from);

        // If copying left
        if to < from {
            // Generate: [>+<-]+[<+>>+<-]>[<+>-]
            // This preserves the original value
            let distance = from - to;

            // First copy to temp: [>+<-]
            self.code.push('[');
            self.code.push('>');
            self.code.push('+');
            self.code.push('<');
            self.code.push('-');
            self.code.push(']');

            // Restore original: >+[<+>>+<-]>[<+>-]
            self.code.push('>');
            self.code.push('+');
            self.code.push('[');
            self.code.push('<');
            self.code.push('+');
            self.code.push('>');
            self.code.push('>');
            self.code.push('+');
            self.code.push('<');
            self.code.push('-');
            self.code.push(']');
            self.code.push('>');
            self.code.push('[');
            self.code.push('<');
            self.code.push('+');
            self.code.push('>');
            self.code.push('-');
            self.code.push(']');
        }
        // If copying right
        else if to > from {
            // Generate: [>+<-]+[>+<<+>-]<[>+<-]
            let distance = to - from;

            // First copy to temp: [>+<-]
            self.code.push('[');
            self.code.push('>');
            self.code.push('+');
            self.code.push('<');
            self.code.push('-');
            self.code.push(']');

            // Restore original: >+[>+<<+>-]<[>+<-]
            self.code.push('>');
            self.code.push('+');
            self.code.push('[');
            self.code.push('>');
            self.code.push('+');
            self.code.push('<');
            self.code.push('<');
            self.code.push('+');
            self.code.push('>');
            self.code.push('-');
            self.code.push(']');
            self.code.push('<');
            self.code.push('[');
            self.code.push('>');
            self.code.push('+');
            self.code.push('<');
            self.code.push('-');
            self.code.push(']');
        }

        self.ptr = to;
    }

    fn generate_expression(&mut self, expression: &Expression) {
        #[cfg(debug_assertions)]
        dbg!(&expression);

        match expression {
            Expression::Number(x) => {
                self.set_new_value(*x);
            }
            Expression::Assignment { name, value } => {
                if self.variables.get(name).is_none() {
                    match *value.clone() {
                        Expression::Number(x) => {
                            let mem = self.allocate_memory(1);
                            self.variables.insert(name.clone(), mem);
                            self.move_to(mem);
                            self.generate_expression(value);
                        }
                        Expression::Array(x) => {
                            let mem = self.allocate_memory(x.len());
                            self.variables.insert(name.clone(), mem);
                            for (i, value) in x.iter().enumerate() {
                                self.move_to(mem + i);
                                self.generate_expression(value);
                            }
                        }
                        _ => {}
                    }
                } else {
                    // Move to memory location
                    self.move_to(*self.variables.get(name).unwrap());

                    // Set value
                    self.add_value(3);
                }
            }
            _ => {}
        }
    }
}
