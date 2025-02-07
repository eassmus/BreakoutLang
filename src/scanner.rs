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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    Func,
    Out,
    Kerchow,
    Bar,
    Define,
    Punch,
    Kick,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PreToken {
    DEL(Delimeter),
    KW(Keyword),
    OP(Operator),
    EOL,
}

const TOKEN_MAP: Map<&str, PreToken> = phf_map! {
"," => PreToken::DEL(Delimeter::Comma),
"(" => PreToken::DEL(Delimeter::LPar),
")" => PreToken::DEL(Delimeter::RPar),
"." => PreToken::DEL(Delimeter::Dot),
"+" => PreToken::OP(Operator::Plus),
"-" => PreToken::OP(Operator::Minus),
"*" => PreToken::OP(Operator::Mult),
"/" => PreToken::OP(Operator::Div),
"|" => PreToken::KW(Keyword::Bar),
"punch" => PreToken::KW(Keyword::Punch),
"kick" => PreToken::KW(Keyword::Kick),
"=>" => PreToken::KW(Keyword::Kerchow),
":=" => PreToken::KW(Keyword::Define),
"func" => PreToken::KW(Keyword::Func),
"out" => PreToken::KW(Keyword::Out)};

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
    let re = Regex::new("(\".*\"|\\(|\\|\\+|\\-|\\*|/|,|:=|=>)").unwrap();
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
            self.lines_stack.push(line.to_owned());
        }
        Ok(())
    }

    pub fn get_next_line(&mut self) -> Option<String> {
        self.lines_stack.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.lines_stack.is_empty()
    }
}
