#![allow(dead_code)]
use crate::primitives::{Bool, Float, Int, Str};
use crate::scanner::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(Int),
    Float(Float),
    String(Str),
    Bool(Bool),
    Void,
}
impl Literal {
    pub fn get_type(&self) -> Type {
        match self {
            Literal::Integer(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::Str,
            Literal::Bool(_) => Type::Bool,
            Literal::Void => Type::NoType,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Integer(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Void => write!(f, "void"),
        }
    }
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

fn parse_literal(s: String, desired_type: Option<Type>) -> Result<Token, ParsingError> {
    if s.starts_with("\"") && s.ends_with("\"") {
        Ok(Token::Lit(Literal::String(Str::new(
            s[1..s.len() - 1].to_string(),
        ))))
    } else if s.parse::<i64>().is_ok() && desired_type.unwrap_or(Type::Int) == Type::Int {
        Ok(Token::Lit(Literal::Integer(Int::new(s.parse().unwrap()))))
    } else if s.parse::<f64>().is_ok() {
        Ok(Token::Lit(Literal::Float(Float::new(s.parse().unwrap()))))
    } else if s == "true" || s == "false" {
        if s == "true" {
            Ok(Token::Lit(Literal::Bool(Bool::new(true))))
        } else {
            Ok(Token::Lit(Literal::Bool(Bool::new(false))))
        }
    } else {
        panic!("where is my literal??")
    }
}

fn parse_symbol(s: String) -> Result<Token, ParsingError> {
    Ok(Token::Symb(Symbol::new(s)))
}

fn parse_word(s: String, desired_type: Option<Type>) -> Result<Token, ParsingError> {
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
        || s.starts_with(".")
        || s == "true"
        || s == "false"
    {
        parse_literal(s, desired_type)
    } else {
        Ok(Token::Symb(Symbol::new(s)))
    }
}

fn parse_line(line: &str) -> Result<Vec<Token>, ParsingError> {
    let mut out: Vec<Token> = Vec::new();
    let pre_tokens = tokenize_line(line.to_string());
    let mut desired_type: Option<Type> = None;
    for token in pre_tokens {
        if let PreTokenized::T(PreToken::TYPE(t)) = token {
            desired_type = Some(t);
        }
        match token {
            PreTokenized::T(t) => out.push(Token::Lang(t)),
            PreTokenized::S(s) => {
                out.push(parse_word(s, desired_type)?);
            }
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
    }
    Ok(out)
}
