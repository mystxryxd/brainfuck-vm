use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::types::{Ast, Token};

pub struct Interpreter {
    code: String,
    memory: [u8; 3000],
    mp: usize,
    curr_index: usize,
}

impl Interpreter {
    pub fn new(code: String) -> Self {
        Self {
            code,
            memory: [0; 3000],
            mp: 0,
            curr_index: 0,
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut lexer: Lexer = Lexer::new(self.code.clone());

        lexer.get_tokens()
    }

    fn compile_ast(&mut self, tokens: Vec<Token>) -> Vec<Ast> {
        let mut parser: Parser = Parser::new(tokens);

        parser.parse()
    }

    fn run_instruction(&mut self, instruction: &Ast) {
        match instruction {
            Ast::INCMP => self.mp += 1,
            Ast::DECMP => self.mp -= 1,
            Ast::INCVAL => self.memory[self.mp] += 1,
            Ast::DECVAL => self.memory[self.mp] -= 1,
            Ast::BRANCH { inner } => {
                while self.memory[self.mp] != 0 {
                    for inst in inner {
                        self.run_instruction(inst);
                    }
                }
            }
            Ast::WRITE => print!("{}", self.memory[self.mp] as char),
            _ => todo!(),
        }
    }

    pub fn run(&mut self) {
        let tokens: Vec<Token> = self.tokenize();
        let ast: Vec<Ast> = self.compile_ast(tokens);

        while self.curr_index < ast.len() {
            self.run_instruction(&ast[self.curr_index]);

            self.curr_index += 1
        }
    }
}
