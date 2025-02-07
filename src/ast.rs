#![allow(dead_code)]
use crate::parser::Literal;
use crate::parser::ParsingError;
use crate::parser::Symbol;
use crate::parser::Token;
use crate::scanner::Delimeter;
use crate::scanner::Keyword;
use crate::scanner::PreToken;

trait Consumer {
    fn consume(&mut self, token_stream: &mut Vec<Token>) -> Result<(), ParsingError>;
}

enum Data {
    Literal(Literal),
    Bytes(Vec<u8>),
}

trait Eval {
    fn eval(&self) -> Data;
}

struct Definition {
    symb_id: usize,
    eval: Box<dyn Eval>,
}

struct FuncStage {
    inputs: Vec<usize>,
    stages: Vec<Definition>,
    outputs: Vec<Box<dyn Eval>>,
    output_id: usize,
}

struct Func {
    symb_id: usize,
    intial_inps: Vec<usize>,
    stages: Vec<FuncStage>,
}

struct AST {
    funcs: Vec<Func>,
}

struct TokenStream {
    tokens: Vec<Token>,
    symbol_id: usize,
    id_name_table: Vec<String>,
}
impl TokenStream {
    fn pop(&mut self) -> Option<Token> {
        self.tokens.pop()
    }
    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    fn get_symbol_id(&mut self, name: &str) -> usize {
        self.id_name_table.push(name.to_string());
        self.symbol_id += 1;
        self.symbol_id - 1
    }
}

fn func_inps_consume(
    token_stream: &mut TokenStream,
    ast: &mut AST,
) -> Result<Vec<Symbol>, ParsingError> {
    let mut inps = vec![];
    while let Some(next_token) = token_stream.pop() {
        if next_token == Token::Lang(PreToken::EOL) {
            break;
        }
        if next_token == Token::Lang(PreToken::DEL(Delimeter::Comma))
            || next_token == Token::Lang(PreToken::DEL(Delimeter::LPar))
            || next_token == Token::Lang(PreToken::DEL(Delimeter::RPar))
        {
            continue;
        }
        if let Token::Symb(new_symbol) = next_token {
            if inps.contains(&new_symbol) {
                return Err(ParsingError {
                    line: 0,
                    message: "Duplicate function_input".to_string(),
                });
            }
            inps.push(new_symbol);
        } else {
            return Err(ParsingError {
                line: 0,
                message: "Unexpected token".to_string(),
            });
        }
    }
    for item in &inps {
        let symb_id = token_stream.get_symbol_id(&item.name());
        ast.funcs.last_mut().unwrap().intial_inps.push(symb_id);
    }
    Ok(inps)
}

fn func_stage_consume(
    token_stream: &mut TokenStream,
    ast: &mut AST,
    inps: &mut Vec<Symbol>,
) -> Result<bool, ParsingError> {
    Ok(true)
}

fn func_consume(token_stream: &mut TokenStream, ast: &mut AST) -> Result<(), ParsingError> {
    if let Some(Token::Symb(symb)) = token_stream.pop() {
        let symb_id = token_stream.get_symbol_id(&symb.name());
        let def_symb = token_stream.pop();
        if def_symb != Some(Token::Lang(PreToken::KW(Keyword::Define))) {
            return Err(ParsingError {
                line: 0,
                message: "Unexpected token in func def, expected :=".to_string(),
            });
        }
        ast.funcs.push(Func {
            symb_id,
            intial_inps: vec![],
            stages: vec![],
        });
        let mut legal_symbs: Vec<Symbol> = func_inps_consume(token_stream, ast)?;
        while func_stage_consume(token_stream, ast, &mut legal_symbs)? {}
    } else {
        return Err(ParsingError {
            line: 0,
            message: "Unexpected token in func def".to_string(),
        });
    }
    Ok(())
}

fn base_consumer(token_stream: &mut TokenStream, ast: &mut AST) -> Result<(), ParsingError> {
    while !token_stream.is_empty() {
        let next_token = token_stream.pop();
        match next_token {
            Some(Token::Lang(PreToken::KW(Keyword::Func))) => func_consume(token_stream, ast)?,
            _ => {
                return Err(ParsingError {
                    line: 0,
                    message: "Unexpected token".to_string(),
                })
            }
        }
    }
    Ok(())
}

fn gen_ast(mut inp: Vec<Token>) -> Result<AST, ParsingError> {
    inp.reverse();
    let mut stream: TokenStream = TokenStream {
        tokens: inp,
        symbol_id: 0,
        id_name_table: vec![],
    };
    let mut ast = AST { funcs: vec![] };
    base_consumer(&mut stream, &mut ast)?;
    Ok(ast)
}
