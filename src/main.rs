use ast_generator::generate_ast;
mod ast_generator;
mod errors;
mod expressions;
mod globalstate;
mod parser;
mod primitives;
mod scanner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "test.bo";
    let out = parser::parse(path);
    let mut global_state = globalstate::GlobalState::new();
    generate_ast(&mut out.unwrap(), &mut global_state)?;
    let output = global_state.eval_main();
    println!("{}", output.unwrap());
    Ok(())
}
