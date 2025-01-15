use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f\n]+")] // Skip whitespace
#[logos(skip r"//[^\n]*")] // Skip single-line comments
#[logos(skip r"/\*(?:[^*]|\*[^/])*\*/")] // Skip multi-line comments
pub enum Token {
    // Operators
    #[token("=")]
    Assign,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,

    // #[token("+")]
    // Plus,
    // #[token("-")]
    // Minus,
    // #[token("*")]
    // Multiply,
    // #[token("/")]
    // Divide,

    // Brackets
    #[token("[")]
    SquareOpen,
    #[token("]")]
    SquareClose,

    #[token("(")]
    RoundOpen,
    #[token(")")]
    RoundClose,

    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,

    // Separators
    // #[token(",")]
    // Comma,
    #[token(";")]
    Semicolon,

    // Keywords
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Iterators
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("..")]
    DoubleDot,

    #[token("!")]
    Not,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(u8),

    #[regex(r"'(.|\\n|\\d)'", |lex| lex.slice().chars().nth(1).map(|c| c as u8))]
    Char(u8),

    #[regex(r#""[^"]*""#, |lex| {
        let slice = lex.slice();
        Some(slice[1..slice.len()-1].to_string())
    })]
    String(String),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    #[regex(r"\+|\-|\*|\/|%", |lex| lex.slice().to_string())]
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexedToken {
    pub token: Token,
    pub range: Range<usize>,
    pub line: String,
    pub chars_before: usize,
}

impl Into<Token> for IndexedToken {
    fn into(self) -> Token {
        self.token
    }
}

pub fn tokenize_indexed(input: &str) -> Vec<IndexedToken> {
    // Add newline so line_text_after doesn't panic
    let input = input.to_owned() + "\n";
    let input = input.replace("\t", &*" ".repeat(8));
    let output = Token::lexer(&*input).spanned().into_iter().map(|(x, span)| {
        let line_number = input[..span.end].chars().filter(|&x| x == '\n').count() + 1;
        let line_text_before = input[..span.end].lines().last().unwrap();
        let line_text_after = input[span.end..].lines().next().unwrap();
        match x {
            Ok(x) => IndexedToken {
                token: x,
                range: span.start..span.end,
                line: line_text_before.to_owned() + line_text_after,
                chars_before: line_text_before.len(),
            },
            Err(e) => {
                eprintln!("Failed lexing `{}` on line {}:", input[span].to_string(), line_number);
                eprintln!("{}{}", line_text_before, line_text_after);
                eprintln!("{}^", " ".repeat(line_text_before.len() - 1));
                std::process::exit(1)
            }
        }
    }).collect();
    #[cfg(debug_assertions)]
    dbg!(&output);
    output
}

pub fn tokenize(input: &str) -> Vec<Token> {
    tokenize_indexed(input).into_iter().map(|x| x.token).collect()
}

impl Token {
    pub fn is_literal(&self) -> bool {
        match self {
            Token::Char(_) | Token::False | Token::True | Token::String(_) | Token::Integer(_) => {
                true
            }
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(_) => true,
            _ => false,
        }
    }
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
