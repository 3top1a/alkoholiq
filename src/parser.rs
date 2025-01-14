use crate::ast::Expression;
use crate::lexer::Token;

fn parse_expression(input: Vec<Token>) -> Option<Expression> {
    if input.is_empty() {
        return None;
    }

    // Simple literal
    if input.len() == 1 {
        match input.iter().next() {
            Some(Token::Integer(value)) => {
                return Some(Expression::Number(*value));
            }
            Some(Token::String(value)) => {
                return Some(Expression::Array(value.as_bytes().iter().map(|x| Expression::Number(*x)).collect()))
            }
            _ => {},
        };
    }

    let mut line = input.iter();

    match line.next().unwrap() {
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
        _ => {}
    }

    eprintln!("Unexpected: {:?}", input);
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