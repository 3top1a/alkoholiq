use crate::ast::Expression;
use crate::lexer::Token;

fn repeat(x: Expression, y: u8) -> Vec<Expression> {
    (0..y).map(|_| x.clone()).collect()
}

fn parse_array(input: Vec<Token>) -> Option<Vec<Expression>> {
    let mut line = input.iter();
    let mut output = vec![];
    
    while let Some(token) = line.next() {
        match token {
            Token::CurlyOpen => {
                let rest = line.take_while(|x| *x != Token::CurlyClose).collect::<Vec<_>>();
                let value = parse_array(rest)?;
                output.push(Expression::Array(value));
            }
            _ => {
                let value = parse_literal(token.clone())?;
                output.push(value);
            }
        }
    }

    Some(output)
}

fn parse_expression(input: Vec<Token>) -> Option<Expression> {
    if input.is_empty() {
        return None;
    }

    // Simple literal
    if input.len() == 1 {
        let literal = input.iter().next().unwrap().clone();
        if let Some(value) = parse_literal(literal) {
            return Some(value);
        }
    }

    let mut line = input.iter();

    match line.next().unwrap() {
        Token::CurlyOpen => {
            // Anything between the curly braces is a closure
            let rest = line.map(|x| x.clone()).take_while(|x| *x != Token::CurlyClose).collect::<Vec<_>>();
            let closure = parse_expression(rest)?;
            return Some(Expression::Closure { body: vec![closure] });
        }

        Token::Identifier(name) => {
            let next = line.next();
            // Assignment
            if let Some(Token::Assign) = next {
                let rest = line.map(|x| x.clone()).collect::<Vec<_>>();

                // The value of the assignment is also an expression
                let value = parse_expression(rest)?;

                return Some(Expression::Assignment {
                    name: name.clone(),
                    value: Box::new(value),
                });
            }
        }

        Token::SquareOpen => {
            if line.rposition(|x| *x == Token::SquareClose).is_none() {
                panic!("Mismatched brackets");
            }
            let rest = line.map(|x| x.clone()).take_while(|x| *x != Token::SquareClose).collect::<Vec<_>>();

            // Normal array
            let normal = rest.iter().position(|x| *x == Token::Semicolon).is_none();
            return if normal {

            } else {
                // Cloned array with semicolon
                assert_eq!(rest.len(), 3);
                assert_eq!(rest[1], Token::Semicolon);
                let item = parse_literal(rest[0].clone()).unwrap();
                let count = parse_literal(rest[2].clone()).unwrap();
                let count = match count {
                    Expression::Number(x) => x,
                    _ => panic!("Expected number"),
                };

                Some(Expression::Array(repeat(item, count)))
            };
        }

        _ => {}
    }

    eprintln!("Unexpected: {:?}", input);
    None
}

fn parse_literal(literal: Token) -> Option<Expression> {
    match literal {
        Token::Integer(value) => {
            return Some(Expression::Number(value));
        }
        Token::String(value) => {
            return Some(Expression::Array(value.as_bytes().iter().map(|x| Expression::Number(*x)).collect()))
        }
        Token::Char(value) => {
            return Some(Expression::Number(value));
        }
        Token::Identifier(name) => {
            return Some(Expression::Identifier(name.clone()));
        }
        Token::True => {
            return Some(Expression::Number(1));
        }
        Token::False => {
            return Some(Expression::Number(0));
        }
        _ => {}
    };
    None
}

pub fn parse(input: Vec<Token>) -> Expression {
    let mut outer = vec![];
    let split = input.split(|x| *x == Token::Newline);

    for line in split {
        let line = parse_expression(line.to_vec());
        if let Some(line) = line {
            outer.push(line);
        }
    }

    Expression::Closure { body: outer }
}
