use crate::types::{Ast, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    curr_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens, curr_index: 0}
    }

    fn parse_branch(&mut self) -> Ast {
        let mut inner_scope: Vec<Ast> = Vec::new();

        while self.tokens.get(self.curr_index + 1).is_some_and(|token: &Token| token.token_type != TokenType::RBRACKET) {
            self.curr_index += 1;

            inner_scope.push(self.parse_next());
        }

        self.curr_index += 1; // skip the RBRACKET token

        Ast::BRANCH { inner: inner_scope }
    }

    fn parse_token(&mut self, token: Token) -> Ast {        
        match token.token_type {
            TokenType::PLUS => Ast::INCVAL,
            TokenType::MINUS => Ast::DECVAL,
            TokenType::RIGHT => Ast::INCMP,
            TokenType::LEFT => Ast::DECMP,
            TokenType::READ => Ast::READ,
            TokenType::WRITE => Ast::WRITE,
            _ => panic!("Illegal token: {:?}", token),
        }
    }

    fn parse_next(&mut self) -> Ast {
        let token: Token = self.tokens[self.curr_index];

        match token.token_type {
            TokenType::LBRACKET => self.parse_branch(),
            _ => self.parse_token(token)
        }
    }

    pub fn parse(&mut self) -> Vec<Ast> {
        let mut instructions: Vec<Ast> = vec![];

        while self.curr_index < self.tokens.len() {
            instructions.push(self.parse_next());

            self.curr_index += 1;
        };

        instructions
    }
}