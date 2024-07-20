use crate::ast::engine::Engine;
use crate::ast::lexer::tokenize;
use crate::ast::paser::parse;

pub struct VM {
    engine: Engine,
}

impl VM {
    pub fn new() -> Self {
        VM {
            engine: Engine::new(),
        }
    }

    pub fn execute(&mut self, script: &str) {
        let tokens = tokenize(script).expect("Failed to tokenize script").1;
        let exprs = parse(&tokens).expect("Failed to parse tokens");
        self.engine.evaluate(exprs);
    }

    pub fn get_variable(&self, name: &str) -> Option<i32> {
        self.engine.get_variable(name)
    }
}
