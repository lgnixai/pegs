use crate::ast::atom::Atom;
use crate::ast::binaryop::BinaryOperation;
use crate::ast::expression::Expression;
use crate::Scope;

impl Expression {
    pub fn evaluate(&self, context: &Scope) -> Result<Expression, String> {
        match self {
            Expression::Atom(Atom::Variable(var_name)) => {
                if let Some(value) = context.get_variable(var_name) {
                    value.evaluate(context)
                } else {
                    Err(format!("Undefined variable: {}", var_name))
                }
            },
            Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
            Expression::BinaryOperation(op, left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;

                match (left_val, right_val) {
                    (Expression::Atom(Atom::Integer(l)), Expression::Atom(Atom::Integer(r))) => {
                        match op {
                            BinaryOperation::Plus => Ok(Expression::Atom(Atom::Integer(l + r))),
                            BinaryOperation::Minus => Ok(Expression::Atom(Atom::Integer(l - r))),
                            BinaryOperation::Times => Ok(Expression::Atom(Atom::Integer(l * r))),
                            BinaryOperation::Divide => Ok(Expression::Atom(Atom::Integer(l / r))),
                            _ => Err("Unsupported binary operation".to_string()),
                        }
                    }
                    (Expression::Atom(Atom::Double(l)), Expression::Atom(Atom::Double(r))) => {
                        match op {
                            BinaryOperation::Plus => Ok(Expression::Atom(Atom::Double(l + r))),
                            BinaryOperation::Minus => Ok(Expression::Atom(Atom::Double(l - r))),
                            BinaryOperation::Times => Ok(Expression::Atom(Atom::Double(l * r))),
                            BinaryOperation::Divide => Ok(Expression::Atom(Atom::Double(l / r))),
                            _ => Err("Unsupported binary operation".to_string()),
                        }
                    }
                    _ => Err("Type error in binary operation".to_string()),
                }
            }
            Expression::Function(func) => {
                context.set_function(func.name.clone(), *(*func).clone());
                Ok(Expression::Function(func.clone()))
            }
            Expression::Tuple(exprs) => {
                let evaluated_exprs: Result<Vec<Expression>, String> = exprs.iter()
                    .map(|e| e.evaluate(context))
                    .collect();
                Ok(Expression::Tuple(evaluated_exprs?))
            }
            Expression::FunctionCall(name, args) => {
                let func = context.get_function(name)
                    .ok_or_else(|| format!("Function '{}' not found", name))?;
                let evaluated_args: Result<Vec<Expression>, String> = args.iter()
                    .map(|arg| arg.evaluate(context))
                    .collect();
                func.call(evaluated_args?, context)
            }
            _ => Err("Unsupported expression type".to_string()),
        }
    }
}
