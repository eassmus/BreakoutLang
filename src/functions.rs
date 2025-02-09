use crate::expressions::Evaluation;
use crate::parser::Literal;
use crate::parser::Symbol;
use crate::scanner::Type;
use std::cell::RefCell;
use std::collections::HashMap as Map;
use std::rc::Rc;

pub enum Function {
    Simple {
        name: Symbol,
        args: Vec<(Symbol, Type)>,
        body: Option<Box<Evaluation>>,
        return_type: Type,
    },
}
impl Clone for Function {
    fn clone(&self) -> Self {
        match self {
            Function::Simple {
                name,
                args,
                body,
                return_type,
            } => Function::Simple {
                name: name.clone(),
                args: args.clone(),
                body: body.clone(),
                return_type: *return_type,
            },
        }
    }
}
impl Function {
    pub fn set_body(&mut self, b: Option<Box<Evaluation>>) {
        match self {
            Function::Simple {
                name: _,
                args: _,
                body,
                return_type: _,
            } => *body = b,
        }
    }
    pub fn get_name(&self) -> Symbol {
        match self {
            Function::Simple { name, .. } => name.clone(),
        }
    }
    pub fn get_args(&self) -> Vec<(Symbol, Type)> {
        match self {
            Function::Simple { args, .. } => args.clone(),
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            Function::Simple {
                name: _,
                args: _,
                body: _,
                return_type,
            } => *return_type,
        }
    }
    pub fn get_arg_types(&self) -> Vec<Type> {
        match self {
            Function::Simple {
                name: _,
                args,
                body: _,
                return_type: _,
            } => args.iter().map(|(_, t)| *t).collect(),
        }
    }
    pub fn evaluate(
        &self,
        inputs: Vec<(Symbol, Evaluation)>,
        global_vars: &mut Rc<RefCell<Map<Symbol, Evaluation>>>,
        global_funcs: &mut Rc<RefCell<Map<Symbol, Function>>>,
    ) -> Literal {
        match self {
            Function::Simple {
                name: _,
                args,
                body,
                return_type: _,
            } => {
                let mut local_vars: Map<Symbol, Evaluation> = Map::new();
                for (k, v) in global_vars.borrow().iter() {
                    local_vars.insert(k.clone(), (*v).clone());
                }
                for i in 0..args.len() {
                    local_vars.insert(args[i].0.clone(), inputs[i].1.clone());
                }
                body.as_ref().unwrap().evaluate(
                    &mut Rc::new(RefCell::new(local_vars)),
                    &mut global_funcs.clone(),
                )
            }
        }
    }
}
