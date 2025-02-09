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

use std::thread;

const STACK_SIZE: usize = 256 * 1024 * 1024;

fn run() {
    let start = SystemTime::now();
    let path: &str = "test.bo";
    let out = parser::parse(path);
    let mut global_state = globalstate::GlobalState::new();
    let _ = generate_ast(&mut out.unwrap(), &mut global_state);
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
