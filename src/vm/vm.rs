use std::cell::RefCell;
use crate::{Engine, RhaiResultOf, Scope};

impl Engine {

    pub fn run(&mut self, code: &str) -> RhaiResultOf<()> {

        let context = Scope::new();
        let result = self.run_ast(code);


        match result {
            Ok(vars) => {
                println!("Variables: {:?}", vars);

            }
            Err(e) => println!("Error: {}", e),
        }
        Ok(())
    }

    pub fn run_scope(&mut self, code: &str,scope:&mut Scope) -> RhaiResultOf<()> {


        let result = self.run_ast_scope(code,scope);


        match result {
            Ok(vars) => {
                println!("Variables: {:?}", vars);

            }
            Err(e) => println!("Error: {}", e),
        }
        Ok(())
    }
}