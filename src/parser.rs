use crate::ast::{Expression, UnaryOperator};
use crate::lexer::{IndexedToken, Token};
use crate::utils::repeat;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<IndexedToken>,
    current: usize,
    input: String,
}


impl Parser {
    pub fn new(tokens: Vec<IndexedToken>, input: String) -> Self {
        Parser {
            tokens,
            current: 0,
            input,
        }
    }

    fn next(&mut self) -> Option<&IndexedToken> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, token: Token) -> Option<&IndexedToken> {
        if self.peek() == Some(token.into()) {
            self.next()
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).map_or(None, |x| Some(x.clone().token))
    }

    fn ipeek(&self) -> Option<IndexedToken> {
        self.tokens.get(self.current).map_or(None, |x| Some(x.clone()))
    }

    fn check(&self, token: Token) -> bool {
        self.peek() == Some(token)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while !self.is_at_end() {
            if let Some(expr) = self.parse_expression() {
                expressions.push(expr);
            } else {
                let itoken = self.ipeek().unwrap();
                eprintln!("Failed to parse expression {:?}", self.peek().unwrap());
                eprintln!("At line:\n{}", itoken.line);
                eprintln!("{}^", " ".repeat(itoken.chars_before - 1));
                std::process::exit(1);
            }
        }

        expressions
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let itoken = self.ipeek()?;
        let token = itoken.token;

        if token.is_literal() {
            self.advance();
            return Some(Self::parse_literal(token));
        }

        match token {
            Token::Identifier(ident) => {
                self.advance();

                // Try assignment
                if self.consume(Token::Assign).is_some() {
                    let value = self.parse_expression()?;
                    return Some(Expression::Assignment {
                        name: ident.clone(),
                        value: Box::new(value),
                    });
                }

                // Check for function call
                if self.consume(Token::RoundOpen).is_some() {
                    let mut args = Vec::new();
                    while self.peek() != Some(Token::RoundClose) {
                        args.push(self.parse_expression()?);
                    }
                    self.advance();
                    return Some(Expression::Call {
                        name: ident.clone(),
                        args,
                    });
                }

                // Otherwise, it's a path
                Some(Expression::Path(ident.clone()))
            }

            // Negation
            Token::Not => {
                self.advance();
                let operand = self.parse_expression()?;
                Some(Expression::Unary {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand),
                })
            }

            // Array
            Token::SquareOpen => {
                self.advance();

                let first = self.parse_expression()?;

                if self.consume(Token::Semicolon).is_some() {
                    let length = match self.parse_expression()? {
                        Expression::Number(n) => n,
                        _ => {
                            eprintln!("Array length must be a number literal");
                            return None;
                        }
                    };

                    self.consume(Token::SquareClose)?;
                    return Some(Expression::Array(repeat(first, length)));
                }

                let mut elements = vec![first];
                while !self.check(Token::SquareClose) {
                    if self.check(Token::SquareClose) {
                        break;
                    }
                    elements.push(self.parse_expression()?);
                }

                self.consume(Token::SquareClose)?;
                Some(Expression::Array(elements))
            }

            // Another expression
            Token::CurlyOpen => {
                self.advance();
                let mut body = Vec::new();
                while self.peek() != Some(Token::CurlyClose) {
                    body.push(self.parse_expression()?);
                }
                self.advance();
                Some(Expression::Expression {
                    body,
                })
            }

            _ => { None }
        }
    }

    fn parse_literal(lit: Token) -> Expression {
        match lit {
            Token::Integer(num) => Expression::Number(num),
            Token::Char(num) => Expression::Number(num),
            Token::String(s) => Expression::Array(s.bytes().map(|x| Expression::Number(x)).collect()),
            Token::True => Expression::Number(1),
            Token::False => Expression::Number(0),
            _ => panic!("Unexpected literal: {:?}", lit),
        }
    }
}
