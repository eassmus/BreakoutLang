use crate::globalstate::GlobalState;
use crate::parser::Literal;
use crate::parser::Symbol;
use crate::parser::Token;
use crate::primitives::OPERATOR_TYPE_TABLE;
use crate::primitives::{Bool, Int, Str};
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
    Variable(Symbol),
}
impl Evaluation {
    pub fn from_tokens(tokens: &mut Vec<Token>, global_state: &mut GlobalState) -> Self {
        match tokens.pop() {
            Some(Token::Lit(literal)) => Evaluation::Literal(literal),
            Some(Token::Lang(PreToken::OP(op))) => {
                let arg1 = Evaluation::from_tokens(tokens, global_state);
                let arg2 = Evaluation::from_tokens(tokens, global_state);
                Evaluation::PrimOp {
                    op,
                    arg1: Box::new(arg1),
                    arg2: Box::new(arg2),
                }
            }
            Some(Token::Lang(PreToken::EOL)) => {
                panic!()
            }
            Some(Token::Symb(symbol)) => Evaluation::Variable(symbol),
            None => todo!(),
            _ => panic!("{:?}", tokens),
        }
    }
    pub fn evaluate(&self, variables: &mut Rc<RefCell<Map<Symbol, Evaluation>>>) -> Literal {
        match self {
            Evaluation::Literal(literal) => literal.clone(),
            Evaluation::Variable(symbol) => variables
                .borrow()
                .get(symbol)
                .unwrap()
                .evaluate(&mut variables.clone()),
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
                let eval1 = arg1.evaluate(variables);
                let eval2 = arg2.evaluate(variables);
                match op_type {
                    Type::Bool => {
                        if let (Literal::Bool(b1), Literal::Bool(b2)) = (eval1, eval2) {
                            match op {
                                Operator::And => Literal::Bool(Bool::and(b1, b2)),
                                Operator::Or => Literal::Bool(Bool::or(b1, b2)),
                                Operator::Nand => Literal::Bool(Bool::nand(b1, b2)),
                                _ => panic!(),
                            }
                        } else {
                            panic!();
                        }
                    }
                    Type::Int => {
                        if let (Literal::Number(i1), Literal::Number(i2)) = (eval1, eval2) {
                            match op {
                                Operator::Plus => Literal::Number(Int::add(i1, i2)),
                                Operator::Minus => Literal::Number(Int::sub(i1, i2)),
                                Operator::Mult => Literal::Number(Int::mul(i1, i2)),
                                Operator::Div => Literal::Number(Int::div(i1, i2)),
                                Operator::Mod => Literal::Number(Int::rem(i1, i2)),
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
