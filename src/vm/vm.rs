use std::cell::RefCell;
use crate::{Engine, RhaiResultOf, Scope};
use crate::ast::atom::Atom;
use crate::ast::expression::Expression;
use crate::package::math::Math;

impl Engine {

    pub fn run(&mut self, code: &str) -> RhaiResultOf<()> {

        let mut context = Scope::new();
        //context.set_variable("ta".to_string(), Expression::Atom(Atom::Variable("ta".to_string())));
        context.register_library("math", Box::new(Math));

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


        let result = self.run_ast(code);


        match result {
            Ok(vars) => {
                println!("Variables: {:?}", vars);

            }
            Err(e) => println!("Error: {}", e),
        }
        Ok(())
    }
}