use ast_generator::generate_ast;
mod ast_generator;
mod errors;
mod expressions;
mod globalstate;
mod parser;
mod primitives;
mod scanner;

use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = SystemTime::now();
    let path: &str = "test.bo";
    let out = parser::parse(path);
    let mut global_state = globalstate::GlobalState::new();
    generate_ast(&mut out.unwrap(), &mut global_state)?;
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
    Ok(())
}
