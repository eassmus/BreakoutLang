use phf::{phf_map, Map};
use regex::Regex;
use regex_split::RegexSplit;
use std::io::Read;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Delimeter {
    Comma,
    LPar,
    RPar,
    Dot,
    Colon,
    Semicolon,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    Func,
    Kerchow,
    Bar,
    Define,
    Punch,
    Kick,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    NoType,
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Str => write!(f, "str"),
            Type::Bool => write!(f, "bool"),
            Type::NoType => write!(f, "no_type"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Eq,
    Neq,
    Geq,
    Gt,
    Leq,
    Lt,
    And,
    Or,
    Not,
    Nand,
    Concat,
    Cond,
    Floor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PreToken {
    DEL(Delimeter),
    KW(Keyword),
    OP(Operator),
    TYPE(Type),
    EOL,
    COMMENT,
}

const TOKEN_MAP: Map<&str, PreToken> = phf_map! {
"," => PreToken::DEL(Delimeter::Comma),
"(" => PreToken::DEL(Delimeter::LPar),
")" => PreToken::DEL(Delimeter::RPar),
"." => PreToken::DEL(Delimeter::Dot),
":" => PreToken::DEL(Delimeter::Colon),
";" => PreToken::DEL(Delimeter::Semicolon),
"+" => PreToken::OP(Operator::Plus),
"-" => PreToken::OP(Operator::Minus),
"*" => PreToken::OP(Operator::Mult),
"/" => PreToken::OP(Operator::Div),
"%" => PreToken::OP(Operator::Mod),
"==" => PreToken::OP(Operator::Eq),
"!=" => PreToken::OP(Operator::Neq),
">=" => PreToken::OP(Operator::Geq),
">" => PreToken::OP(Operator::Gt),
"<=" => PreToken::OP(Operator::Leq),
"<" => PreToken::OP(Operator::Lt),
"&&" => PreToken::OP(Operator::And),
"||" => PreToken::OP(Operator::Or),
"!" => PreToken::OP(Operator::Not),
"!&&" => PreToken::OP(Operator::Nand),
"concat" => PreToken::OP(Operator::Concat),
"cond" => PreToken::OP(Operator::Cond),
"floor" => PreToken::OP(Operator::Floor),
"|" => PreToken::KW(Keyword::Bar),
"punch" => PreToken::KW(Keyword::Punch),
"kick" => PreToken::KW(Keyword::Kick),
"=>" => PreToken::KW(Keyword::Kerchow),
":=" => PreToken::KW(Keyword::Define),
"func" => PreToken::KW(Keyword::Func),
"int" => PreToken::TYPE(Type::Int),
"float" => PreToken::TYPE(Type::Float),
"str" => PreToken::TYPE(Type::Str),
"bool" => PreToken::TYPE(Type::Bool),
"#" => PreToken::COMMENT,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreTokenized {
    T(PreToken),
    S(String),
}

fn string_to_tokenize(s: &str) -> PreTokenized {
    let res = TOKEN_MAP.get(s);
    match res {
        Some(t) => PreTokenized::T(*t),
        None => PreTokenized::S(s.to_owned()),
    }
}

pub fn tokenize_line(line: String) -> Vec<PreTokenized> {
    let re = Regex::new("(\".*\"|\\(|\\)|\\|\\+|\\-|\\*|/|,|:=|=>|;)").unwrap();
    let mut split: Vec<PreTokenized> = re
        .split_inclusive(line.as_str())
        .flat_map(|s| re.split_inclusive_left(s))
        .flat_map(|s| {
            if re.is_match(s) {
                vec![s]
            } else {
                s.split_whitespace().collect()
            }
        })
        .map(string_to_tokenize)
        .filter(|t| t != &PreTokenized::T(PreToken::COMMENT))
        .filter(|t| t != &PreTokenized::T(PreToken::DEL(Delimeter::Comma)))
        .filter(|t| t != &PreTokenized::T(PreToken::DEL(Delimeter::Semicolon)))
        .collect();
    split.push(PreTokenized::T(PreToken::EOL));
    split
}

pub struct Scanner {
    lines_stack: Vec<String>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            lines_stack: Vec::new(),
        }
    }

    pub fn load_file(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        for line in contents.lines().rev() {
            if line.starts_with("include") {
                let re = Regex::new("(include )(.+)").unwrap();
                let include_path = re.captures(line).unwrap().get(2).unwrap().as_str();
                self.load_file(include_path)?;
            } else {
                self.lines_stack.push(line.to_owned());
            }
        }
        Ok(())
    }

    pub fn get_next_line(&mut self) -> Option<String> {
        self.lines_stack.pop()
    }
}
