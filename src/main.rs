use std::env;

mod interpreter;
mod lexer;
mod parser;
mod types;

fn main() {
    let code: String = env::args().nth(1).expect("Are you retarded?");

    let mut vm: interpreter::Interpreter = interpreter::Interpreter::new(code);
    vm.run();
}
