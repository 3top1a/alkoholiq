use crate::tokens::{IndexedToken, Token};
use logos::Logos;

pub fn tokenize_indexed(input: &str) -> Vec<IndexedToken> {
    // Add newline so line_text_after doesn't panic
    let input = input.to_owned() + "\n";
    let input = input.replace("\t", &*" ".repeat(8));
    let output = Token::lexer(&*input)
        .spanned()
        .into_iter()
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
                Err(e) => {
                    eprintln!(
                        "Failed lexing `{}` on line {}:",
                        input[span].to_string(),
                        line_number
                    );
                    eprintln!("{}{}", line_text_before, line_text_after);
                    eprintln!("{}^", " ".repeat(line_text_before.len() - 1));
                    std::process::exit(1)
                }
            }
        })
        .collect();
    output
}

pub fn tokenize(input: &str) -> Vec<Token> {
    tokenize_indexed(input)
        .into_iter()
        .map(|x| x.token)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = "//Est\nvar = 42";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("var".to_string()),
                Token::Assign,
                Token::Integer(42),
            ]
        );
    }

    #[test]
    fn test_array() {
        let input = "array = [1 2 3]";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("array".to_string()),
                Token::Assign,
                Token::SquareOpen,
                Token::Integer(1),
                Token::Integer(2),
                Token::Integer(3),
                Token::SquareClose,
            ]
        );
    }

    #[test]
    fn test_string() {
        let input = r#"str* = "Hello""#;
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("str".to_string()),
                Token::Identifier("*".to_string()),
                Token::Assign,
                Token::String("Hello".to_string()),
            ]
        );
    }
}
