use crate::expressions::Evaluation;
use crate::globalstate::GlobalState;
use crate::parser::{Literal, Token};
use crate::primitives::OPERATOR_TYPE_TABLE;
use crate::scanner::{Keyword, PreToken, Type};
use std::error::Error;

fn consume_evaluation(
    tokens: &mut Vec<Token>,
    global_state: &mut GlobalState,
    desired_type: Type,
) -> Result<Evaluation, Box<dyn Error>> {
    let eval = Evaluation::from_tokens(tokens, global_state);
    match eval {
        Evaluation::Literal(ref lit) => {
            match lit {
                Literal::Number(_) => {
                    if Type::Int == desired_type {
                        return Ok(eval);
                    }
                }
                Literal::String(_) => {
                    if Type::Str == desired_type {
                        return Ok(eval);
                    }
                }
                Literal::Bool(_) => {
                    if Type::Bool == desired_type {
                        return Ok(eval);
                    }
                }
            }
            return Err("Invalid type".into());
        }
        Evaluation::PrimOp { ref op, .. } => {
            let op_type = {
                let mut op_type = None;
                for op_pair in OPERATOR_TYPE_TABLE.iter() {
                    if op_pair.0 == *op {
                        op_type = Some(op_pair.1);
                        break;
                    }
                }
                op_type
            };
            if let Some(t) = op_type {
                if t == desired_type {
                    return Ok(eval);
                }
            }
            Err("Invalid type".into())
        }
    }
}

pub fn generate_ast(
    tokens: &mut Vec<Token>,
    global_state: &mut GlobalState,
) -> Result<(), Box<dyn Error>> {
    tokens.reverse();
    while !tokens.is_empty() {
        let mut line_start_token = tokens.pop();
        while line_start_token == Some(Token::Lang(PreToken::EOL)) {
            line_start_token = tokens.pop();
        }
        match line_start_token {
            Some(Token::Lang(PreToken::TYPE(t))) => {
                if let Some(Token::Symb(func_name)) = tokens.pop() {
                    let def_symbol = tokens.pop(); // def symbol
                    if def_symbol != Some(Token::Lang(PreToken::KW(Keyword::Define))) {
                        return Err("Invalid token, wanted :=".into());
                    }
                    let expression = consume_evaluation(tokens, global_state, t)?;
                    global_state.add_variable(func_name, expression)?;
                } else {
                    return Err("Invalid token, wanted a function name".into());
                }
            }
            None => break,
            _ => {
                return Err(format!("Invalid token, got {:?}", line_start_token).into());
            }
        }
    }
    Ok(())
}
