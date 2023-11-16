use std::env;

mod lexer;
mod types;
mod parser;
mod interpreter;

fn main() {
    let code: String = env::args().nth(1).expect("Are you retarded?");

    let mut vm: interpreter::Interpreter = interpreter::Interpreter::new(code);
    vm.run();
}
