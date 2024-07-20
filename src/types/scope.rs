use std::cell::RefCell;
use std::collections::HashMap;
use crate::ast::expression::Expression;
use crate::ast::function::Function;


#[derive(Debug, Clone)]
pub struct Scope {
    pub(crate) variables: RefCell<HashMap<String, Expression>>,

    functions: RefCell<HashMap<String, Function>>,

}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: RefCell::new(Default::default()),
            functions: RefCell::new(Default::default()),
        }
    }

    pub fn set_variable(&self, name: String, value: Expression) {
        self.variables.borrow_mut().insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<Expression> {
        self.variables.borrow().get(name).cloned()
    }

    pub fn set_function(&self, name: String, function: Function) {
        self.functions.borrow_mut().insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        self.functions.borrow().get(name).cloned()
    }
}
