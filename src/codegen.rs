use std::collections::HashMap;
use std::str::from_utf8;
use crate::ast::Expression;

// Codegen notes for myself:
// - Make the codegen also do comments, they will get removed in the optimization step
// - Functions: https://brainfuck.org/function_tutorial.b
// - 


/// Because we do not yet know the length of functions, we need to make a list of all the 
/// basic instructions and their purpose. This will be used to generate the code after initial
/// analysis.
enum AbstractInstruction {
        
}

