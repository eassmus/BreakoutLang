use crate::scanner::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    pub fn new(name: String) -> Symbol {
        Symbol { name }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Lang(PreToken),
    Symb(Symbol),
    Lit(Literal),
}

#[derive(Debug, Clone)]
pub struct ParsingError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error on line {}:\n\t{}", self.line, self.message)
    }
}
impl Error for ParsingError {}

fn parse_literal(s: String) -> Result<Token, ParsingError> {
    if s.starts_with("\"") && s.ends_with("\"") {
        Ok(Token::Lit(Literal::String(s[1..s.len() - 1].to_string())))
    } else if s.parse::<i64>().is_ok() {
        Ok(Token::Lit(Literal::Number(s.parse().unwrap())))
    } else {
        Err(ParsingError {
            line: 0,
            message: format!("Invalid literal {}", s),
        })
    }
}

fn parse_symbol(s: String) -> Result<Token, ParsingError> {
    Ok(Token::Symb(Symbol::new(s)))
}

fn parse_word(s: String) -> Result<Token, ParsingError> {
    if s.starts_with("\"")
        || s.starts_with("0")
        || s.starts_with("1")
        || s.starts_with("2")
        || s.starts_with("3")
        || s.starts_with("4")
        || s.starts_with("5")
        || s.starts_with("6")
        || s.starts_with("7")
        || s.starts_with("8")
        || s.starts_with("9")
    {
        parse_literal(s)
    } else {
        Ok(Token::Symb(Symbol::new(s)))
    }
}

fn parse_line(line: &str) -> Result<Vec<Token>, ParsingError> {
    let mut out: Vec<Token> = Vec::new();
    let pre_tokens = tokenize_line(line.to_string());
    for token in pre_tokens {
        match token {
            PreTokenized::T(t) => out.push(Token::Lang(t)),
            PreTokenized::S(s) => out.push(parse_word(s)?),
        }
    }
    Ok(out)
}

pub fn parse(path: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut scanner = Scanner::new();
    scanner.load_file(path)?;
    let mut out: Vec<Token> = Vec::new();
    while let Some(line) = scanner.get_next_line() {
        out.append(&mut parse_line(&line)?);
        println!("{}", line);
    }
    Ok(out)
}
