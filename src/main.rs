use std::env;

mod lexer;
mod types;
mod parser;
mod interpreter;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let code: String = env::args().nth(1).expect("Are you retarded?");

    let mut vm = interpreter::Interpreter::new(code);
    vm.run();
}
