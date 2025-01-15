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
    #[token(";")]
    Semicolon,

    // Keywords
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("if")]
    If,
    #[token("else")]
    Else,

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
    // Because math ops are also functions
    Identifier(String),
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

    pub fn inner_string(&self) -> Option<String> {
        match self {
            Token::Identifier(x) => Some(x.clone()),
            Token::String(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexedToken {
    pub token: Token,
    pub range: Range<usize>,
    pub line: String,
    pub line_number: usize,
    pub chars_before: usize,
}

impl IndexedToken {
    pub fn new_hidden(token: Token) -> Self {
        IndexedToken {
            token,
            range: 0..0,
            line: "".to_string(),
            line_number: 0,
            chars_before: 0,
        }
    }
}
