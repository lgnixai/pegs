use std::collections::HashMap;
use crate::vm::parser::{Expression, Function};

#[derive(Debug, Clone)]

pub struct Context {
    variables: HashMap<String, Expression>,
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Expression) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Expression> {
        self.variables.get(name)
    }

    pub fn set_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}
