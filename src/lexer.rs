use crate::types::{Token, TokenType};

pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {code}
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        for char in self.code.chars() {
            tokens.push(match char {
                '>' => Token {token_type: TokenType::RIGHT, value: Some(char)},
                '<' => Token {token_type: TokenType::LEFT, value: Some(char)},
                '+' => Token {token_type: TokenType::PLUS, value: Some(char)},
                '-' => Token {token_type: TokenType::MINUS, value: Some(char)},
                '[' => Token {token_type: TokenType::LBRACKET, value: Some(char)},
                ']' => Token {token_type: TokenType::RBRACKET, value: Some(char)},
                '.' => Token {token_type: TokenType::WRITE, value: Some(char)},
                ',' => Token {token_type: TokenType::READ, value: Some(char)},
                _ => {
                    panic!("Invalid character {char}");
                }
            })
        }

        tokens
    }
}