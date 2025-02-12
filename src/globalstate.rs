#![allow(dead_code)]
use crate::errors::ProgramError;
use crate::expressions::Evaluation;
use crate::functions::Function;
use crate::parser::{Literal, Symbol};
use crate::scanner::Type;
use std::cell::RefCell;
use std::collections::HashMap as Map;
use std::rc::Rc;

struct SymbolTable {
    token_to_id: Map<Symbol, usize>,
    id_to_token: Map<usize, Symbol>,
    count: usize,
}
impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            count: 0,
            token_to_id: Map::new(),
            id_to_token: Map::new(),
        }
    }
    fn add(&mut self, symbol: Symbol) -> Result<usize, ProgramError> {
        if self.token_to_id.contains_key(&symbol) {
            return Err(ProgramError {
                message: symbol.clone().to_string() + " already exists",
            });
        }
        self.count += 1;
        self.token_to_id.insert(symbol.clone(), self.count);
        self.id_to_token.insert(self.count, symbol);
        Ok(self.count)
    }
    fn get_id(&self, symbol: Symbol) -> Option<usize> {
        self.token_to_id.get(&symbol).cloned()
    }
    fn get_symbol(&self, id: usize) -> Option<Symbol> {
        self.id_to_token.get(&id).cloned()
    }
    fn symbol_exists(&self, symbol: Symbol) -> bool {
        self.token_to_id.contains_key(&symbol)
    }
    fn id_exists(&self, id: usize) -> bool {
        self.id_to_token.contains_key(&id)
    }
}

pub struct GlobalState {
    symbol_table: SymbolTable,
    variables: Rc<RefCell<Map<Symbol, Evaluation>>>,
    variable_override: Rc<RefCell<Map<Symbol, Type>>>,
    func_table: Rc<RefCell<Map<Symbol, Function>>>,
    main_evaluation: Option<Evaluation>,
}

impl GlobalState {
    pub fn new() -> Self {
        let symbol_table = SymbolTable::new();
        let variables = Rc::new(RefCell::new(Map::new()));
        let variable_override = Rc::new(RefCell::new(Map::new()));
        let func_table = Rc::new(RefCell::new(Map::new()));
        GlobalState {
            symbol_table,
            variables,
            variable_override,
            func_table,
            main_evaluation: None,
        }
    }
    pub fn add_variable(&mut self, name: Symbol, value: Evaluation) -> Result<(), ProgramError> {
        self.symbol_table.add(name.clone())?;
        if name.name() == "main" {
            self.main_evaluation = Some(value);
        } else {
            self.variables.borrow_mut().insert(name, value);
        }
        Ok(())
    }
    pub fn add_function(&mut self, name: Symbol, function: Function) -> Result<(), ProgramError> {
        self.symbol_table.add(name.clone())?;
        self.func_table.borrow_mut().insert(name, function);
        Ok(())
    }
    pub fn set_function_body(&mut self, name: Symbol, body: Evaluation) {
        self.func_table
            .borrow_mut()
            .get_mut(&name)
            .unwrap()
            .set_body(Some(Box::new(body)));
    }
    pub fn set_function_body_breakout(&mut self, name: Symbol, body: Function) {
        self.func_table.borrow_mut().insert(name, body.clone());
    }
    pub fn get_args(&self, symbol: &Symbol) -> Vec<Type> {
        self.func_table
            .borrow()
            .get(symbol)
            .map(|f| f.get_arg_types())
            .unwrap()
    }
    pub fn is_function(&self, symbol: &Symbol) -> bool {
        self.func_table
            .borrow()
            .get(symbol)
            .map(|_| true)
            .unwrap_or(false)
    }
    pub fn is_variables(&self, symbol: &Symbol) -> bool {
        self.variables
            .borrow()
            .get(symbol)
            .map(|_| true)
            .unwrap_or(false)
    }
    pub fn override_variables(&mut self, symbol: Symbol, value: Type) {
        self.variable_override.borrow_mut().insert(symbol, value);
    }
    pub fn clear_overrides(&mut self) {
        self.variable_override.borrow_mut().clear();
    }
    pub fn get_type(&self, symbol: &Symbol) -> Type {
        if let Some(id) = self.variable_override.borrow().get(symbol) {
            return *id;
        } else if let Some(id) = self.variables.borrow().get(symbol) {
            return id.get_type();
        } else if let Some(id) = self.func_table.borrow().get(symbol) {
            return id.get_type();
        }
        panic!("Type not found, symbol: {}", symbol);
    }
    pub fn eval_main(mut self) -> Result<Literal, ProgramError> {
        let eval = match self.main_evaluation {
            Some(e) => e,
            None => {
                println!("No main function found");
                panic!()
            }
        };
        let literal = eval.evaluate(&mut self.variables, &mut self.func_table);
        Ok(literal)
    }
}
