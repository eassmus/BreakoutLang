use ast_generator::generate_ast;
mod ast_generator;
mod errors;
mod expressions;
mod functions;
mod globalstate;
mod parser;
mod primitives;
mod scanner;

use std::time::SystemTime;

use std::env;
use std::thread;

const STACK_SIZE: usize = 256 * 1024 * 1024;

fn run() {
    let start = SystemTime::now();

    let args: Vec<String> = env::args().collect();
    let path: &str = args[1].as_str();
    let out = parser::parse(path);
    let mut global_state = globalstate::GlobalState::new();
    let ast = generate_ast(&mut out.unwrap(), &mut global_state);
    if ast.is_err() {
        println!("{}", ast.unwrap_err());
        return;
    }
    let end = SystemTime::now();
    println!(
        "Parsed Source in: {}ms\n",
        end.duration_since(start).unwrap().as_millis()
    );

    let exec_start = SystemTime::now();

    let output = global_state.eval_main();

    let exec_end = SystemTime::now();

    println!("{}", output.unwrap());
    println!(
        "\nExecuted in: {}ms\n",
        exec_end.duration_since(exec_start).unwrap().as_millis()
    );
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    child.join().unwrap();
}
