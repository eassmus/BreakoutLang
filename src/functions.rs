use crate::expressions::Evaluation;
use crate::parser::Literal;
use crate::parser::Symbol;
use crate::scanner::Type;
use std::cell::RefCell;
use std::collections::HashMap as Map;
use std::rc::Rc;
use std::thread;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RunType {
    Regular,
    Thread,
}
impl PartialOrd for RunType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RunType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else if self == &RunType::Regular {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionStage {
    assignments: Vec<(Symbol, Box<Evaluation>, RunType)>,
}
impl FunctionStage {
    pub fn new(mut a: Vec<(Symbol, Box<Evaluation>, RunType)>) -> Self {
        a.sort_by(|x, y| x.2.cmp(&y.2));
        FunctionStage {
            assignments: a.to_vec(),
        }
    }
    pub fn evaluate(
        &self,
        vars: &mut Rc<RefCell<Map<Symbol, Evaluation>>>,
        funcs: &mut Rc<RefCell<Map<Symbol, Function>>>,
    ) {
        let mut handles = Vec::new();
        for (name, eval_to, run_type) in &self.assignments {
            match run_type {
                RunType::Regular => {
                    let eval = eval_to.evaluate(vars, funcs);
                    vars.borrow_mut()
                        .insert(name.clone(), Evaluation::Literal(eval));
                }
                RunType::Thread => {
                    let mut new_vars = Map::new();
                    let mut new_funcs = Map::new();
                    for x in vars.borrow().iter() {
                        new_vars.insert(x.0.clone(), x.1.clone());
                    }
                    for x in funcs.borrow().iter() {
                        new_funcs.insert(x.0.clone(), x.1.clone());
                    }
                    let move_eval_to = eval_to.clone();
                    let handle = thread::spawn(move || {
                        let v = new_vars;
                        let f = new_funcs;
                        move_eval_to
                            .evaluate(&mut Rc::new(RefCell::new(v)), &mut Rc::new(RefCell::new(f)))
                    });
                    handles.push((name, handle));
                }
            }
        }
        for (name, handle) in handles {
            vars.borrow_mut()
                .insert(name.clone(), Evaluation::Literal(handle.join().unwrap()));
        }
    }
}

pub enum Function {
    Simple {
        name: Symbol,
        args: Vec<(Symbol, Type)>,
        body: Option<Box<Evaluation>>,
        return_type: Type,
    },
    Breakout {
        name: Symbol,
        args: Vec<(Symbol, Type)>,
        stages: Vec<FunctionStage>,
        final_eval: Box<Evaluation>,
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
            Function::Breakout {
                name,
                args,
                stages,
                final_eval,
                return_type,
            } => Function::Breakout {
                name: name.clone(),
                args: args.clone(),
                stages: stages.to_vec(),
                final_eval: final_eval.clone(),
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
            _ => todo!(),
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            Function::Simple { return_type, .. } => *return_type,
            Function::Breakout { return_type, .. } => *return_type,
        }
    }
    pub fn get_arg_types(&self) -> Vec<Type> {
        match self {
            Function::Simple { args, .. } => args.iter().map(|(_, t)| *t).collect(),
            Function::Breakout { args, .. } => args.iter().map(|(_, t)| *t).collect(),
        }
    }
    pub fn evaluate(
        &self,
        global_vars: &mut Rc<RefCell<Map<Symbol, Evaluation>>>,
        global_funcs: &mut Rc<RefCell<Map<Symbol, Function>>>,
    ) -> Literal {
        match self {
            Function::Simple { body, .. } => body
                .as_ref()
                .unwrap()
                .evaluate(&mut global_vars.clone(), &mut global_funcs.clone()),
            Function::Breakout {
                stages, final_eval, ..
            } => {
                for stage in stages {
                    stage.evaluate(&mut global_vars.clone(), &mut global_funcs.clone());
                }
                final_eval.evaluate(&mut global_vars.clone(), &mut global_funcs.clone())
            }
        }
    }
}
