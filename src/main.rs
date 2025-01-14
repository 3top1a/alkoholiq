use std::io::Read;

mod lexer;
mod ast;
mod parser;

fn main() {
    let input = std::env::args().nth(1).map_or_else(
        || {
            // Else take from stdin
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        },
        |file| {
            // If argument is a file, read the file
            std::fs::read_to_string(file).unwrap()
        }
    );

    let tokens = lexer::tokenize(&input);

    let ast = parser::parse(tokens);

    println!("{:#?}", ast);
}
