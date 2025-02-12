use crate::errors::*;
use crate::expressions::Evaluation;
use crate::functions::{Function, FunctionStage, RunType};
use crate::globalstate::GlobalState;
use crate::parser::{Symbol, Token};
use crate::scanner::{Delimeter, Keyword, PreToken, Type};
use std::error::Error;

fn consume_evaluation(
    tokens: &mut Vec<Token>,
    global_state: &mut GlobalState,
    desired_type: Type,
) -> Result<Evaluation, Box<dyn Error>> {
    let eval = Evaluation::from_tokens(tokens, global_state);
    if eval.get_type() != desired_type {
        return Err(Box::new(TypeError {
            message: "Type mismatch".to_string(),
            expected: desired_type,
            found: eval.get_type(),
        }));
    }
    Ok(eval)
}

fn consume_function_stage(
    tokens: &mut Vec<Token>,
    global_state: &mut GlobalState,
) -> Result<FunctionStage, Box<dyn Error>> {
    let mut curr_token = tokens.pop();
    let mut assignments: Vec<(Symbol, Box<Evaluation>, RunType)> = Vec::new();
    while curr_token != Some(Token::Lang(PreToken::KW(Keyword::Kerchow))) {
        match curr_token {
            Some(Token::Lang(PreToken::TYPE(t))) => {
                let name = match tokens.pop() {
                    Some(Token::Symb(name)) => name,
                    x => panic!("{:?} is not a valid token, {:#?}", x, tokens),
                };
                let def_symbol = tokens.pop(); // def symbol
                if def_symbol != Some(Token::Lang(PreToken::KW(Keyword::Define))) {
                    return Err("Invalid token, wanted :=".into());
                }
                let body = consume_evaluation(tokens, global_state, t)?;
                if body.get_type() != t {
                    return Err(Box::new(TypeError {
                        message: "Type mismatch".to_string(),
                        expected: t,
                        found: body.get_type(),
                    }));
                }
                global_state.override_variables(name.clone(), t);
                let run_type = match tokens.last().unwrap() {
                    Token::Lang(PreToken::KW(Keyword::Bar)) => {
                        tokens.pop();
                        match tokens.pop() {
                            Some(Token::Lang(PreToken::KW(Keyword::Kick))) => {
                                tokens.pop();
                                RunType::Thread
                            }
                            _ => panic!(),
                        }
                    }
                    _ => RunType::Regular,
                };
                assignments.push((name, Box::new(body), run_type));
            }
            Some(Token::Lang(PreToken::EOL)) => {}
            Some(Token::Lang(PreToken::KW(Keyword::Bar))) => {}
            _ => panic!("{:?} is not a valid token, {:#?}", curr_token, tokens),
        }
        curr_token = tokens.pop();
    }
    tokens.push(curr_token.unwrap());
    Ok(FunctionStage::new(assignments))
}

fn consume_function(
    name: Symbol,
    tokens: &mut Vec<Token>,
    global_state: &mut GlobalState,
    desired_type: Type,
) -> Result<(), Box<dyn Error>> {
    let mut args: Vec<(Symbol, Type)> = Vec::new();
    let mut next_token = tokens.pop();
    while next_token != Some(Token::Lang(PreToken::KW(Keyword::Kerchow)))
        && next_token != Some(Token::Lang(PreToken::KW(Keyword::Bar)))
    {
        if let Some(Token::Symb(var_name)) = next_token {
            let colon_symbol = tokens.pop(); // def symbol
            if colon_symbol != Some(Token::Lang(PreToken::DEL(Delimeter::Colon))) {
                return Err("Invalid token, wanted :".into());
            }
            if let Some(Token::Lang(PreToken::TYPE(t))) = tokens.pop() {
                args.push((var_name.clone(), t));
                global_state.override_variables(var_name, t);
            } else {
                return Err("Invalid token, wanted a type".into());
            }
        }
        next_token = tokens.pop();
    }
    let func = Function::Simple {
        name: name.clone(),
        args: args.clone(),
        body: None,
        return_type: desired_type,
    };
    global_state.add_function(name.clone(), func)?;
    match next_token {
        Some(Token::Lang(PreToken::KW(Keyword::Kerchow))) => {
            let body = consume_evaluation(tokens, global_state, desired_type)?;
            if body.get_type() != desired_type {
                return Err(Box::new(TypeError {
                    message: "Type mismatch".to_string(),
                    expected: desired_type,
                    found: body.get_type(),
                }));
            }
            global_state.set_function_body(name, body);
            global_state.clear_overrides();
        }
        Some(Token::Lang(PreToken::KW(Keyword::Bar))) => {
            let mut stages: Vec<FunctionStage> = Vec::new();
            loop {
                let func_stage = consume_function_stage(tokens, global_state)?;
                stages.push(func_stage);
                let _kerchow = tokens.pop();
                if tokens.last().unwrap() != &Token::Lang(PreToken::EOL) {
                    // we are terminal
                    let last_eval = consume_evaluation(tokens, global_state, desired_type)?;
                    let t = last_eval.get_type();
                    if t != desired_type {
                        return Err(Box::new(TypeError {
                            message: "Type mismatch".to_string(),
                            expected: desired_type,
                            found: last_eval.get_type(),
                        }));
                    };
                    let func = Function::Breakout {
                        name: name.clone(),
                        args: args.clone(),
                        stages: stages.to_vec(),
                        final_eval: Box::new(last_eval),
                        return_type: t,
                    };
                    global_state.set_function_body_breakout(name.clone(), func);
                    break;
                }
            }
        }
        _ => todo!(),
    };
    global_state.clear_overrides();

    Ok(())
}

pub fn generate_ast(
    tokens: &mut Vec<Token>,
    global_state: &mut GlobalState,
) -> Result<(), Box<dyn Error>> {
    tokens.reverse();
    while !tokens.is_empty() {
        let mut line_start_token = tokens.pop();
        if line_start_token == Some(Token::Lang(PreToken::COMMENT)) {
            while line_start_token != Some(Token::Lang(PreToken::EOL)) {
                line_start_token = tokens.pop();
            }
        }
        while line_start_token == Some(Token::Lang(PreToken::EOL)) {
            line_start_token = tokens.pop();
        }
        match line_start_token {
            Some(Token::Lang(PreToken::TYPE(t))) => {
                let next_token = tokens.pop();
                if let Some(Token::Symb(func_name)) = next_token {
                    let def_symbol = tokens.pop(); // def symbol
                    if def_symbol != Some(Token::Lang(PreToken::KW(Keyword::Define))) {
                        return Err("Invalid token, wanted :=".into());
                    }
                    let expression = consume_evaluation(tokens, global_state, t)?;
                    global_state.add_variable(func_name, expression)?;
                } else if let Some(Token::Lang(PreToken::KW(Keyword::Func))) = next_token {
                    let func_name = tokens.pop();
                    if let Some(Token::Symb(func_name)) = func_name {
                        let def_symbol = tokens.pop(); // def symbol
                        if def_symbol != Some(Token::Lang(PreToken::KW(Keyword::Define))) {
                            return Err("Invalid token, wanted :=".into());
                        }
                        consume_function(func_name.clone(), tokens, global_state, t)?;
                        // function adds to global state
                    }
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
