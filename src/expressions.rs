use crate::functions::Function;
use crate::globalstate::GlobalState;
use crate::parser::Literal;
use crate::parser::Symbol;
use crate::parser::Token;
use crate::primitives::OPERATOR_TYPE_TABLE;
use crate::primitives::{Bool, Int, Str};
use crate::scanner::Delimeter;
use crate::scanner::Operator;
use crate::scanner::PreToken;
use crate::scanner::Type;
use std::cell::RefCell;
use std::collections::HashMap as Map;
use std::rc::Rc;

#[derive(Debug)]
pub enum Evaluation {
    Literal(Literal),
    PrimOp {
        op: Operator,
        arg1: Box<Evaluation>,
        arg2: Box<Evaluation>,
    },
    Variable(Symbol, Type),
    Conditional {
        cond: Box<Evaluation>,
        then: Box<Evaluation>,
        otherwise: Box<Evaluation>,
    },
    FuncCall {
        name: Symbol,
        args: Vec<Evaluation>,
        return_type: Type,
    },
}
impl Clone for Evaluation {
    fn clone(&self) -> Self {
        match self {
            Evaluation::Literal(literal) => Evaluation::Literal(literal.clone()),
            Evaluation::PrimOp { op, arg1, arg2 } => Evaluation::PrimOp {
                op: *op,
                arg1: Box::new(*arg1.clone()),
                arg2: Box::new(*arg2.clone()),
            },
            Evaluation::Variable(symbol, t) => Evaluation::Variable(symbol.clone(), *t),
            Evaluation::Conditional {
                cond,
                then,
                otherwise,
            } => Evaluation::Conditional {
                cond: Box::new(*cond.clone()),
                then: Box::new(*then.clone()),
                otherwise: Box::new(*otherwise.clone()),
            },
            Evaluation::FuncCall {
                name,
                args,
                return_type,
            } => Evaluation::FuncCall {
                name: name.clone(),
                args: args.clone(),
                return_type: *return_type,
            },
        }
    }
}
impl Evaluation {
    pub fn from_tokens(tokens: &mut Vec<Token>, global_state: &mut GlobalState) -> Self {
        match tokens.pop() {
            Some(Token::Lit(literal)) => Evaluation::Literal(literal),
            Some(Token::Lang(PreToken::OP(Operator::Cond))) => {
                let cond = Evaluation::from_tokens(tokens, global_state);
                let then = Evaluation::from_tokens(tokens, global_state);
                let otherwise = Evaluation::from_tokens(tokens, global_state);
                if cond.get_type() != Type::Bool || then.get_type() != otherwise.get_type() {
                    panic!()
                }
                Evaluation::Conditional {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                }
            }
            Some(Token::Lang(PreToken::OP(op))) => {
                let arg1 = Evaluation::from_tokens(tokens, global_state);
                let arg2 = Evaluation::from_tokens(tokens, global_state);
                Evaluation::PrimOp {
                    op,
                    arg1: Box::new(arg1),
                    arg2: Box::new(arg2),
                }
            }
            Some(Token::Lang(PreToken::DEL(Delimeter::LPar))) => {
                let eval = Evaluation::from_tokens(tokens, global_state);
                let next = tokens.pop().unwrap();
                if next != Token::Lang(PreToken::DEL(Delimeter::RPar)) {
                    panic!("{:?}", next);
                };
                eval
            }
            Some(Token::Symb(symbol)) => {
                let t: Type = global_state.get_type(&symbol);
                if global_state.is_function(&symbol) {
                    let needed_types = global_state.get_args(&symbol);
                    let mut args: Vec<Evaluation> = Vec::new();
                    for needed_type in needed_types {
                        let eval = Evaluation::from_tokens(tokens, global_state);
                        if eval.get_type() != needed_type {
                            panic!()
                        }
                        args.push(eval);
                    }
                    Evaluation::FuncCall {
                        name: symbol,
                        args,
                        return_type: t,
                    }
                } else {
                    Evaluation::Variable(symbol, t)
                }
            }
            Some(Token::Lang(PreToken::EOL)) => {
                panic!()
            }
            None => todo!(),
            _ => panic!("{:?}", tokens),
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            Evaluation::Literal(ref lit) => match lit {
                Literal::Number(_) => Type::Int,
                Literal::String(_) => Type::Str,
                Literal::Bool(_) => Type::Bool,
            },
            Evaluation::PrimOp { ref op, .. } => {
                let mut op_type = None;
                for op_pair in OPERATOR_TYPE_TABLE.iter() {
                    if op_pair.0 == *op {
                        op_type = Some(op_pair.1);
                        break;
                    }
                }
                op_type.unwrap()
            }
            Evaluation::FuncCall { return_type: t, .. } => *t,
            Evaluation::Variable(_, t) => *t,
            Evaluation::Conditional { then, .. } => then.get_type(),
        }
    }
    pub fn evaluate(
        &self,
        variables: &mut Rc<RefCell<Map<Symbol, Evaluation>>>,
        functions: &mut Rc<RefCell<Map<Symbol, Function>>>,
    ) -> Literal {
        match self {
            Evaluation::Literal(literal) => literal.clone(),
            Evaluation::Variable(symbol, _) => {
                let out = variables
                    .borrow()
                    .get(symbol)
                    .unwrap()
                    .evaluate(&mut variables.clone(), &mut functions.clone());
                out
            }
            Evaluation::Conditional {
                cond,
                then,
                otherwise,
            } => {
                let cond = cond.evaluate(variables, functions);
                if let Literal::Bool(b) = cond {
                    if b.get() {
                        then.evaluate(variables, functions)
                    } else {
                        otherwise.evaluate(variables, functions)
                    }
                } else {
                    panic!()
                }
            }
            Evaluation::FuncCall { name, args, .. } => {
                let func = (*functions.borrow().get(name).unwrap()).clone();
                match func {
                    Function::Simple {
                        args: ref needed_args,
                        ..
                    } => {
                        let mut give_vars: Rc<RefCell<Map<Symbol, Evaluation>>> =
                            Rc::new(RefCell::new(Map::new()));
                        let mut max_inst: Map<Symbol, usize> = Map::new();
                        for (sym, eval) in variables.borrow().iter() {
                            give_vars.borrow_mut().insert(sym.clone(), eval.clone());
                            if sym.instance() >= *max_inst.get(sym).unwrap_or(&0) {
                                max_inst.insert(sym.clone(), sym.instance());
                            }
                        }
                        for (sym, to_eval) in needed_args.iter().zip(args.iter()) {
                            let new_sym = Symbol::new_instance(sym.0.name(), sym.0.instance());
                            let eval = to_eval.evaluate(&mut give_vars, functions);
                            give_vars
                                .borrow_mut()
                                .insert(new_sym, Evaluation::Literal(eval));
                        }
                        func.evaluate(&mut give_vars, functions)
                    }
                }
            }
            Evaluation::PrimOp { op, arg1, arg2 } => {
                let op_type = {
                    let mut op_type = None;
                    for op_pair in OPERATOR_TYPE_TABLE.iter() {
                        if op_pair.0 == *op {
                            op_type = Some(op_pair.1);
                            break;
                        }
                    }
                    op_type
                }
                .unwrap();
                let eval1 = arg1.evaluate(variables, functions);
                let eval2 = arg2.evaluate(variables, functions);
                match op_type {
                    Type::Bool => {
                        if let (Literal::Bool(b1), Literal::Bool(b2)) = (&eval1, &eval2) {
                            return match op {
                                Operator::And => Literal::Bool(Bool::and(*b1, *b2)),
                                Operator::Or => Literal::Bool(Bool::or(*b1, *b2)),
                                Operator::Nand => Literal::Bool(Bool::nand(*b1, *b2)),
                                Operator::Eq => Literal::Bool(Bool::eq(*b1, *b2)),
                                Operator::Neq => Literal::Bool(Bool::neq(*b1, *b2)),
                                _ => panic!(),
                            };
                        } else if let (Literal::Number(i1), Literal::Number(i2)) = (&eval1, &eval2)
                        {
                            return match op {
                                Operator::Gt => Literal::Bool(Int::gt(*i1, *i2)),
                                Operator::Geq => Literal::Bool(Int::geq(*i1, *i2)),
                                Operator::Lt => Literal::Bool(Int::lt(*i1, *i2)),
                                Operator::Leq => Literal::Bool(Int::leq(*i1, *i2)),
                                Operator::Eq => Literal::Bool(Int::eq(*i1, *i2)),
                                Operator::Neq => Literal::Bool(Int::neq(*i1, *i2)),
                                _ => panic!(),
                            };
                        } else if let (Literal::String(s1), Literal::String(s2)) = (eval1, eval2) {
                            return match op {
                                Operator::Eq => Literal::Bool(Str::eq(s1, s2)),
                                Operator::Neq => Literal::Bool(Str::neq(s1, s2)),
                                _ => panic!(),
                            };
                        }
                        panic!()
                    }
                    Type::Int => {
                        if let (Literal::Number(i1), Literal::Number(i2)) = (eval1, eval2) {
                            match op {
                                Operator::Plus => Literal::Number(Int::add(i1, i2)),
                                Operator::Minus => Literal::Number(Int::sub(i1, i2)),
                                Operator::Mult => Literal::Number(Int::mul(i1, i2)),
                                Operator::Div => Literal::Number(Int::div(i1, i2)),
                                Operator::Mod => Literal::Number(Int::rem(i1, i2)),
                                Operator::Gt => Literal::Bool(Int::gt(i1, i2)),
                                Operator::Geq => Literal::Bool(Int::geq(i1, i2)),
                                Operator::Lt => Literal::Bool(Int::lt(i1, i2)),
                                Operator::Leq => Literal::Bool(Int::leq(i1, i2)),
                                Operator::Eq => Literal::Bool(Int::eq(i1, i2)),
                                Operator::Neq => Literal::Bool(Int::neq(i1, i2)),
                                _ => panic!(),
                            }
                        } else {
                            panic!();
                        }
                    }
                    Type::Str => {
                        if let (Literal::String(s1), Literal::String(s2)) = (eval1, eval2) {
                            match op {
                                Operator::Concat => {
                                    Literal::String(Str::concat(s1.clone(), s2.clone()))
                                }
                                Operator::Eq => Literal::Bool(Str::eq(s1, s2)),
                                Operator::Neq => Literal::Bool(Str::neq(s1, s2)),
                                _ => panic!(),
                            }
                        } else {
                            panic!();
                        }
                    }
                }
            }
        }
    }
}
