use crate::ast::expression::Expression;
use crate::ast::function::Function;
use crate::ast::stmt::Statement;
use crate::Scope;

impl Function {
    pub fn call(&self, args: Vec<Expression>, context: &Scope) -> Result<Expression, String> {
        if args.len() != self.parameters.len() {
            return Err("Argument count mismatch".to_string());
        }

        let mut local_context = Scope::new();
        local_context.variables.borrow_mut().extend(context.variables.borrow().clone());

        for (param, arg) in self.parameters.iter().zip(args) {
            let evaluated_arg = arg.evaluate(&mut local_context)?;
            local_context.set_variable(param.name.clone(), evaluated_arg);
        }

        for statement in &self.body.statements {
            match statement {
                Statement::VariableDeclaration(name, expr) => {
                    let value = expr.evaluate(&mut local_context)?;
                    local_context.set_variable(name.clone(), value);
                }
                _ => {}
            }
        }

        self.body.return_expr.evaluate(&mut local_context)
    }
}