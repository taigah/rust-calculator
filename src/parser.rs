use super::lexer;
use std::vec::Vec;
use lexer::Token;
use lexer::TokenType;
use lexer::TokenFamily;

#[derive(PartialEq)]
enum Associativity {
    Left,
    Right
}

fn operator_associativity(op: &Token) -> Associativity {
    match op.type_ {
        TokenType::Plus | TokenType::Multiply | TokenType::Minus | TokenType::Divide => Associativity::Left,
        _ => Associativity::Left
    }
}

fn operator_priority(op: &Token) -> u8 {
    match op.type_ {
        TokenType::Multiply | TokenType::Divide => 10,
        TokenType::Plus | TokenType::Minus => 5,
        _ => 0
    }
}

fn to_npi(source: Vec<Token>) -> Result<Vec<lexer::Token>, String> {
    let mut stack : Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    for token in source.iter() {
        let token = token.clone();
        match token.family() {
            TokenFamily::Literal => output.push(token),
            TokenFamily::Operator => {
                let o1assoc = operator_associativity(&token);
                let o1prio = operator_priority(&token);
                while !stack.is_empty() && stack.last().unwrap().family() == TokenFamily::Operator {
                    let o2 = stack.last().unwrap();
                    let o2prio = operator_priority(&o2);
                    if (o1assoc == Associativity::Left && o1prio <= o2prio)
                        || (o1assoc == Associativity::Right && o1prio < o2prio) {
                        output.push(stack.pop().unwrap());
                        continue;
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }
            TokenFamily::Punctuation => {
                match token.type_ {
                    TokenType::LeftParenthesis => stack.push(token),
                    TokenType::RightParenthesis => {
                        let mut poped = stack.pop().unwrap();
                        while poped.type_ != TokenType::LeftParenthesis {
                            output.push(poped);
                            poped = stack.pop().unwrap();
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    while stack.is_empty() != true {
        let token = stack.pop().unwrap();
        if token.family() != TokenFamily::Operator {
            return Err(String::from("L'expression est mal parenthésée"));
        }
        output.push(token);
    }
    Ok(output)
}

fn eval(tokens: Vec<lexer::Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in tokens.iter() {
        match token.family() {
            TokenFamily::Literal => {
                let num: f64 = token.value.parse().expect("Le nombre n'a pas pu être parsé correctement");
                stack.push(num);
            },
            TokenFamily::Operator => {
                let b = stack.pop();
                let a = stack.pop();
                if a == None || b == None {
                    return Err(String::from("L'expression est mal formée"));
                }
                let a = a.unwrap();
                let b = b.unwrap();
                let result = match token.type_ {
                    TokenType::Plus => a + b,
                    TokenType::Minus => a - b,
                    TokenType::Multiply => a * b,
                    TokenType::Divide => a / b,
                    _ => 0.0
                };
                stack.push(result);
            },
            TokenFamily::Punctuation => return Err(String::from("De la poncutation a été détectée là où elle ne devrait pas être, uh oh !"))
        }
    }
    if stack.len() != 1 {
        return Err(String::from("Il ne doit rester qu'un nombre dans la stack, eh"));
    }
    Ok(*stack.last().unwrap())
}

pub fn parse(tokens: Vec<lexer::Token>) {
    if tokens.is_empty() {
        return
    }
    match to_npi(tokens) {
        Ok(output) => {
            if output.is_empty() {
                return
            }
            match eval(output) {
                Ok(result) => println!("{}", result),
                Err(err) => println!("Evaluation Error:\n{}", err)
            }
        }
        Err(err) => println!("Syntax Error:\n{}", err)
    }
}