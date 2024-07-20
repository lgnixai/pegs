use crate::vm::parser::{Expression, Function, Statement};
use crate::vm::parser::Expression::{Atom, BinaryOperation};
use crate::vm::vm::Context;


impl Statement {
    pub fn execute(&self, context: &mut Context) -> Result<(), String> {
        match self {
            Statement::VariableDeclaration(name, expr) => {
                let value = expr.evaluate(context)?;
                context.set_variable(name.clone(), value);
                Ok(())
            }
        }
    }
}

impl Function {
    pub fn call(&self, args: Vec<Expression>, context: &mut Context) -> Result<Expression, String> {
        if args.len() != self.parameters.len() {
            return Err("Incorrect number of arguments".to_string());
        }

        let mut local_context = context.clone();
        for (param, arg) in self.parameters.iter().zip(args) {
            local_context.set_variable(param.name.clone(), arg);
        }

        for stmt in &self.body.statements {
            stmt.execute(&mut local_context)?;
        }

        self.body.return_expr.evaluate(&mut local_context)
    }
}
