use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]  // Skip whitespace
#[logos(skip r"//[^\n]*")]    // Skip single-line comments
#[logos(skip r"/\*(?:[^*]|\*[^/])*\*/")]  // Skip multi-line comments
pub enum Token {
    // Operators
    #[token("=")]
    Assign,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,

    // Brackets
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,

    // Separators
    #[token(",")]
    Comma,

    // Keywords
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(u8),

    #[regex(r"'.'", |lex| lex.slice().chars().nth(1).map(|c| c as u8))]
    Char(u8),

    #[regex(r#""[^"]*""#, |lex| {
        let slice = lex.slice();
        Some(slice[1..slice.len()-1].to_string())
    })]
    String(String),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    Token::lexer(input).map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = "var = 42";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![
            Token::Identifier("var".to_string()),
            Token::Assign,
            Token::Integer(42),
        ]);
    }

    #[test]
    fn test_array() {
        let input = "array* = [1, 2, 3]";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![
            Token::Identifier("array".to_string()),
            Token::Multiply,
            Token::Assign,
            Token::BracketOpen,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::Comma,
            Token::Integer(3),
            Token::BracketClose,
        ]);
    }

    #[test]
    fn test_string() {
        let input = r#"str* = "Hello""#;
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![
            Token::Identifier("str".to_string()),
            Token::Multiply,
            Token::Assign,
            Token::String("Hello".to_string()),
        ]);
    }

    #[test]
    fn test_comments() {
        let input = "// This is a comment\nvar = 42\n/* This is a multi-line comment */";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![
            Token::Identifier("var".to_string()),
            Token::Assign,
            Token::Integer(42),
        ]);
    }
}
