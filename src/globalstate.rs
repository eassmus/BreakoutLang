#![allow(dead_code)]
use crate::errors::ProgramError;
use crate::expressions::Evaluation;
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
    main_evaluation: Option<Evaluation>,
}

impl GlobalState {
    pub fn new() -> Self {
        let symbol_table = SymbolTable::new();
        let variables = Rc::new(RefCell::new(Map::new())); //Map::new();
        GlobalState {
            symbol_table,
            variables,
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
    pub fn get_type(&self, symbol: &Symbol) -> Type {
        self.variables.borrow().get(symbol).unwrap().get_type()
    }
    pub fn eval_main(mut self) -> Result<Literal, ProgramError> {
        let eval = self.main_evaluation.unwrap();
        let literal = eval.evaluate(&mut self.variables);
        Ok(literal)
    }
}
