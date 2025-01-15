use std::io::Read;

mod ast;
mod lexer;
mod parser;
mod tokens;
mod utils;

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
            #[cfg(debug_assertions)]
            dbg!("Reading file", &file);
            std::fs::read_to_string(file).unwrap()
        },
    );

    let tokens = lexer::tokenize_indexed(&input);

    let output = tokens.iter().map(|x| x.token.clone()).collect::<Vec<_>>();
    #[cfg(debug_assertions)]
    dbg!(&output);

    let mut parser = parser::Parser::new(tokens, input);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
