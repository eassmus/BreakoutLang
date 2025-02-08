use crate::{scanner::Operator, scanner::Type};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Int {
    value: i64,
}
impl Int {
    pub fn new(value: i64) -> Int {
        Int { value }
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
}
impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
    pub fn nand(a: Bool, b: Bool) -> Bool {
        Bool {
            value: !(a.value && b.value),
        }
    }
}
impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
    pub fn concat(a: Str, b: Str) -> Str {
        Str {
            value: a.value + &b.value,
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
        write!(f, "{}", self.value)
    }
}

pub const OPERATOR_TYPE_TABLE: [(Operator, Type); 9] = [
    (Operator::Plus, Type::Int),
    (Operator::Minus, Type::Int),
    (Operator::Mult, Type::Int),
    (Operator::Div, Type::Int),
    (Operator::Mod, Type::Int),
    (Operator::Or, Type::Bool),
    (Operator::And, Type::Bool),
    (Operator::Nand, Type::Bool),
    (Operator::Concat, Type::Str),
];
