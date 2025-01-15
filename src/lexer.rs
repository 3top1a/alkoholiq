use crate::tokens::{IndexedToken, Token};
use logos::Logos;

pub fn tokenize_indexed(input: &str) -> Vec<IndexedToken> {
    // Add newline so line_text_after doesn't panic
    let input = input.to_owned() + "\n";
    let input = input.replace("\t", &" ".repeat(8));
    let output = Token::lexer(&*input)
        .spanned()
        .map(|(x, span)| {
            let line_number = input[..span.end].chars().filter(|&x| x == '\n').count() + 1;
            let line_text_before = input[..span.end].lines().last().unwrap();
            let line_text_after = input[span.end..].lines().next().unwrap();
            match x {
                Ok(x) => IndexedToken {
                    token: x,
                    range: span.start..span.end,
                    line: line_text_before.to_owned() + line_text_after,
                    chars_before: line_text_before.len(),
                    line_number,
                },
                Err(_) => {
                    eprintln!(
                        "Failed lexing `{}` on line {}:",
                        &input[span],
                        line_number
                    );
                    eprintln!("{}{}", line_text_before, line_text_after);
                    eprintln!("{}^", " ".repeat(line_text_before.len() - 1));
                    std::process::exit(1)
                }
            }
        })
        .collect();
    postprocess(output)
}

#[allow(dead_code)]
pub fn tokenize(input: &str) -> Vec<Token> {
    tokenize_indexed(input)
        .into_iter()
        .map(|x| x.token)
        .collect()
}

fn postprocess(mut input: Vec<IndexedToken>) -> Vec<IndexedToken> {
    for token in input.iter_mut() {
        match token.token {
            Token::String(ref mut x) => {
                *x = x.replace("\\n", "\n");
            }
            _ => {}
        }
    }

    input
}
