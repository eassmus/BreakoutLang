use crate::functions::Function;
use crate::parser::Symbol;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    expressions::Evaluation,
    parser::Literal,
    scanner::{Operator, Type},
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Float {
    value: f64,
}
impl Eq for Float {}
impl Float {
    pub fn new(value: f64) -> Float {
        Float { value }
    }
    pub fn from_int(value: i64) -> Float {
        Float {
            value: value as f64,
        }
    }
    pub fn get(&self) -> f64 {
        self.value
    }
    pub fn add(a: Float, b: Float) -> Float {
        Float {
            value: a.value + b.value,
        }
    }
    pub fn sub(a: Float, b: Float) -> Float {
        Float {
            value: a.value - b.value,
        }
    }
    pub fn mul(a: Float, b: Float) -> Float {
        Float {
            value: a.value * b.value,
        }
    }
    pub fn div(a: Float, b: Float) -> Float {
        Float {
            value: a.value / b.value,
        }
    }
    pub fn eq(a: Float, b: Float) -> Bool {
        Bool {
            value: a.value == b.value,
        }
    }
    pub fn neq(a: Float, b: Float) -> Bool {
        Bool {
            value: a.value != b.value,
        }
    }
    pub fn gt(a: Float, b: Float) -> Bool {
        Bool {
            value: a.value > b.value,
        }
    }
    pub fn lt(a: Float, b: Float) -> Bool {
        Bool {
            value: a.value < b.value,
        }
    }
    pub fn geq(a: Float, b: Float) -> Bool {
        Bool {
            value: a.value >= b.value,
        }
    }
    pub fn leq(a: Float, b: Float) -> Bool {
        Bool {
            value: a.value <= b.value,
        }
    }
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Int {
    value: i64,
}
impl Int {
    pub fn new(value: i64) -> Int {
        Int { value }
    }
    pub fn get(&self) -> i64 {
        self.value
    }
    pub fn from_float(value: f64) -> Int {
        Int {
            value: value as i64,
        }
    }
    pub fn add(a: Int, b: Int) -> Int {
        Int {
            value: a.value + b.value,
        }
    }
    pub fn sub(a: Int, b: Int) -> Int {
        Int {
            value: a.value - b.value,
        }
    }
    pub fn mul(a: Int, b: Int) -> Int {
        Int {
            value: a.value * b.value,
        }
    }
    pub fn div(a: Int, b: Int) -> Int {
        Int {
            value: a.value / b.value,
        }
    }
    pub fn rem(a: Int, b: Int) -> Int {
        Int {
            value: a.value % b.value,
        }
    }
    pub fn gt(a: Int, b: Int) -> Bool {
        Bool {
            value: a.value > b.value,
        }
    }
    pub fn lt(a: Int, b: Int) -> Bool {
        Bool {
            value: a.value < b.value,
        }
    }
    pub fn geq(a: Int, b: Int) -> Bool {
        Bool {
            value: a.value >= b.value,
        }
    }
    pub fn leq(a: Int, b: Int) -> Bool {
        Bool {
            value: a.value <= b.value,
        }
    }
    pub fn eq(a: Int, b: Int) -> Bool {
        Bool {
            value: a.value == b.value,
        }
    }
    pub fn neq(a: Int, b: Int) -> Bool {
        Bool {
            value: a.value != b.value,
        }
    }
}
impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Bool {
    value: bool,
}
impl Bool {
    pub fn new(value: bool) -> Bool {
        Bool { value }
    }
    pub fn get(&self) -> bool {
        self.value
    }
    pub fn or(a: Bool, b: Bool) -> Bool {
        Bool {
            value: a.value || b.value,
        }
    }
    pub fn and(a: Bool, b: Bool) -> Bool {
        Bool {
            value: a.value && b.value,
        }
    }
    pub fn not(a: Bool) -> Bool {
        Bool { value: !a.value }
    }
    pub fn nand(a: Bool, b: Bool) -> Bool {
        Bool {
            value: !(a.value && b.value),
        }
    }
    pub fn eq(a: Bool, b: Bool) -> Bool {
        Bool {
            value: a.value == b.value,
        }
    }
    pub fn neq(a: Bool, b: Bool) -> Bool {
        Bool {
            value: a.value != b.value,
        }
    }
}
impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Str {
    value: String,
}
impl Str {
    pub fn new(value: String) -> Str {
        Str { value }
    }
    pub fn get(&self) -> String {
        self.value.clone()
    }
    pub fn concat(a: Str, b: Str) -> Str {
        Str {
            value: a.value + &b.value,
        }
    }
    pub fn eq(a: Str, b: Str) -> Bool {
        Bool {
            value: a.value == b.value,
        }
    }
    pub fn neq(a: Str, b: Str) -> Bool {
        Bool {
            value: a.value != b.value,
        }
    }
}
impl Clone for Str {
    fn clone(&self) -> Self {
        Str {
            value: self.value.clone(),
        }
    }
}
impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub fn go_to_float(type1: Type, type2: Type) -> Type {
    if type1 == Type::Float || type2 == Type::Float {
        Type::Float
    } else {
        Type::Int
    }
}

pub fn exec_prim_op(
    op: Operator,
    arg1: Box<Evaluation>,
    arg2: Box<Option<Evaluation>>,
    variables: Rc<RefCell<HashMap<Symbol, Evaluation>>>,
    functions: Rc<RefCell<HashMap<Symbol, Function>>>,
) -> Literal {
    let mut eval1 = arg1.evaluate(&mut variables.clone(), &mut functions.clone());
    let mut eval2 = {
        if let Some(a) = arg2.as_ref() {
            a.evaluate(&mut variables.clone(), &mut functions.clone())
        } else {
            Literal::Void
        }
    }; //arg2.evaluate(&mut variables.clone(), &mut functions.clone());
    match (eval1.clone(), eval2.clone()) {
        (Literal::Integer(a), Literal::Float(_)) => {
            eval1 = Literal::Float(Float::from_int(a.get()));
        }
        (Literal::Float(_), Literal::Integer(b)) => {
            eval2 = Literal::Float(Float::from_int(b.get()));
        }
        _ => {}
    }
    match op {
        Operator::Eq => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Bool(Int::eq(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Bool(Float::eq(a, b)),
            (Literal::Bool(a), Literal::Bool(b)) => Literal::Bool(Bool::eq(a, b)),
            (Literal::String(a), Literal::String(b)) => Literal::Bool(Str::eq(a, b)),
            _ => panic!(),
        },
        Operator::Gt => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Bool(Int::gt(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Bool(Float::gt(a, b)),
            _ => panic!(),
        },
        Operator::Lt => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Bool(Int::lt(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Bool(Float::lt(a, b)),
            _ => panic!(),
        },
        Operator::Geq => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Bool(Int::geq(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Bool(Float::geq(a, b)),
            _ => panic!(),
        },
        Operator::Leq => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Bool(Int::leq(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Bool(Float::leq(a, b)),
            _ => panic!(),
        },
        Operator::Neq => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Bool(Int::neq(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Bool(Float::neq(a, b)),
            (Literal::Bool(a), Literal::Bool(b)) => Literal::Bool(Bool::neq(a, b)),
            (Literal::String(a), Literal::String(b)) => Literal::Bool(Str::neq(a, b)),
            _ => panic!(),
        },
        Operator::Or => match (eval1, eval2) {
            (Literal::Bool(a), Literal::Bool(b)) => Literal::Bool(Bool::or(a, b)),
            _ => panic!(),
        },
        Operator::And => match (eval1, eval2) {
            (Literal::Bool(a), Literal::Bool(b)) => Literal::Bool(Bool::and(a, b)),
            _ => panic!(),
        },
        Operator::Not => match eval1 {
            Literal::Bool(a) => Literal::Bool(Bool::not(a)),
            _ => panic!(),
        },
        Operator::Nand => match (eval1, eval2) {
            (Literal::Bool(a), Literal::Bool(b)) => Literal::Bool(Bool::nand(a, b)),
            _ => panic!(),
        },
        Operator::Mod => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Integer(Int::rem(a, b)),
            _ => panic!(),
        },
        Operator::Plus => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Integer(Int::add(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Float(Float::add(a, b)),
            _ => panic!(),
        },
        Operator::Minus => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Integer(Int::sub(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Float(Float::sub(a, b)),
            _ => panic!(),
        },
        Operator::Mult => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Integer(Int::mul(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Float(Float::mul(a, b)),
            _ => panic!(),
        },
        Operator::Div => match (eval1, eval2) {
            (Literal::Integer(a), Literal::Integer(b)) => Literal::Integer(Int::div(a, b)),
            (Literal::Float(a), Literal::Float(b)) => Literal::Float(Float::div(a, b)),
            _ => panic!(),
        },
        Operator::Concat => match (eval1, eval2) {
            (Literal::String(a), Literal::String(b)) => Literal::String(Str::concat(a, b)),
            _ => panic!(),
        },
        Operator::Floor => match eval1 {
            Literal::Float(a) => Literal::Integer(Int::from_float(a.get())),
            _ => panic!(),
        },
        Operator::Cond => panic!("uhhhhh"),
    }
}

pub fn get_prim_op_type(op: Operator, type1: Type, type2: Type) -> Type {
    match op {
        Operator::Eq => Type::Bool,
        Operator::Gt => Type::Bool,
        Operator::Lt => Type::Bool,
        Operator::Geq => Type::Bool,
        Operator::Leq => Type::Bool,
        Operator::Neq => Type::Bool,
        Operator::Or => Type::Bool,
        Operator::And => Type::Bool,
        Operator::Nand => Type::Bool,
        Operator::Not => Type::Bool,
        Operator::Mod => Type::Int,
        Operator::Plus => go_to_float(type1, type2),
        Operator::Minus => go_to_float(type1, type2),
        Operator::Mult => go_to_float(type1, type2),
        Operator::Div => go_to_float(type1, type2),
        Operator::Concat => Type::Str,
        Operator::Cond => panic!("ermmmmm how did we get here?"),
        Operator::Floor => Type::Int,
    }
}
