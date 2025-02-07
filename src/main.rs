mod ast;
mod parser;
mod scanner;

fn main() {
    let path: &str = "test.bo";
    let out = parser::parse(path);
    println!("{:?}", out.unwrap());
}
