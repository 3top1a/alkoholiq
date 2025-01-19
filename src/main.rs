use std::io::Read;

mod ast;
mod codegen;
mod ir;
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

    let normal_tokens = tokens.iter().map(|x| x.token.clone()).collect::<Vec<_>>();
    #[cfg(debug_assertions)]
    dbg!(&normal_tokens);

    let ast = parser::Parser::new(tokens, input).parse();

    #[cfg(debug_assertions)]
    dbg!(&ast);

    let ir = ir::IRGenerator::new().generate(ast);
    dbg!(ir);
}
