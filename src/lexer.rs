use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Int,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenFamily {
    Literal,
    Operator,
    Punctuation
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub type_: TokenType,
    pub value: String
}

impl Token {
    pub fn family(&self) -> TokenFamily {
        match self.type_ {
            TokenType::Plus => TokenFamily::Operator,
            TokenType::Minus => TokenFamily::Operator,
            TokenType::Multiply => TokenFamily::Operator,
            TokenType::Divide => TokenFamily::Operator,
            TokenType::LeftParenthesis => TokenFamily::Punctuation,
            TokenType::RightParenthesis => TokenFamily::Punctuation,
            TokenType::Int => TokenFamily::Literal
        }
    }
}

fn sym_to_token(ch: char) -> Option<Token> {
    let type_: TokenType;
    match ch {
        '+' => type_ = TokenType::Plus,
        '-' => type_ = TokenType::Minus,
        '*' => type_ = TokenType::Multiply,
        '/' => type_ = TokenType::Divide,
        '(' => type_ = TokenType::LeftParenthesis,
        ')' => type_ = TokenType::RightParenthesis,
        _ => return None
    }
    Some(Token {
        type_,
        value: ch.to_string()
    })
}

#[derive(Debug, PartialEq)]
enum TokenizationState {
    ReadingInt,
    None
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut state = TokenizationState::None;
    let mut tokens: Vec<Token> = Vec::new();
    for ch in code.chars() {
        if ch.is_digit(10) {
            if state == TokenizationState::ReadingInt {
                tokens.last_mut().unwrap().value.push(ch);
            } else {
                state = TokenizationState::ReadingInt;
                tokens.push(Token {
                    type_: TokenType::Int,
                    value: ch.to_string()
                });
            }
        } else {
            if state == TokenizationState::ReadingInt {
                state = TokenizationState::None;
            }
            match sym_to_token(ch) {
                Some(token) => {
                    // gestion des opérateurs unaires + et -, on rajoute 0 devant
                    if token.type_ == TokenType::Plus || token.type_ == TokenType::Minus {
                        if tokens.is_empty() {
                            tokens.push(Token {
                                type_: TokenType::Int,
                                value: String::from("0")
                            });
                        } else {
                            let last_token = tokens.last().unwrap();
                            if last_token.type_ == TokenType::LeftParenthesis
                                || last_token.family() == TokenFamily::Operator {
                                tokens.push(Token {
                                    type_: TokenType::Int,
                                    value: String::from("0")
                                });
                            }
                        }
                    }
                    tokens.push(token);
                    continue
                },
                None => {
                    if ch != ' ' && ch != '\n' {
                        let mut err = String::from("Utilisation d'un symbole interdit: ");
                        err.push(ch);
                        return Err(err);
                    }
                }
            }
        }
    }
    Ok(tokens)
}