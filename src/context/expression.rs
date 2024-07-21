use crate::ast::atom::Atom;
use crate::ast::binaryop::BinaryOperation;
use crate::ast::expression::Expression;
use crate::object::object::Object;
use crate::Scope;

impl Expression {
    pub fn evaluate(&self, context: &mut Scope) -> Result<Expression, String> {
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
            Expression::MethodCall(lib_name, method_name, args) => {

                println!("lib_name==={:?},{:?},{:?}",lib_name,method_name,args);


                let args_clone: Vec<Expression> = args.clone();

                println!("111{:?},{:?},{:?}",lib_name,method_name,args);

                let result = context.call_library_function(lib_name, method_name, args_clone);

                println!("result===={:?}",result);

                match result {
                    Ok(result) => Ok(result), // Return the result wrapped in Ok
                    Err(err) => Err(err),     // Return the error
                }
                // match result {


                //     Ok(expression) => println!("Function call succeeded with result: {:?}", expression),
                //     Err(err) => eprintln!("Function call failed with error: {:?}", err),
                // }
                //     let arg = arg.evaluate(context)?;
                // if let Some(result) = context.call_library_function(lib_name, method_name, args) {
                //     Ok(Expression::Atom(Atom::Double(result)))
                // } else {
                //     Err(format!("Unsupported method call {}.{}", lib_name, method_name))
                // }
                // if lib_name == "import" {
                //     if let Some(library_name) = args.get(0) {
                //         if let Expression::Atom(Atom::String(lib_name)) = library_name {
                //             context.import_library(lib_name);
                //             Ok(Expression::Atom(Atom::String(format!("Imported library: {}", lib_name))))
                //         } else {
                //             Err("Import library name must be a string".to_string())
                //         }
                //     } else {
                //         Err("Import statement must specify a library name".to_string())
                //     }
                // } else if let Some(arg) = args.get(0) {
                //     let arg = arg.evaluate(context)?;
                //     match arg {
                //         Expression::Atom(Atom::Double(x)) => {
                //             if let Some(result) = context.call_library_function(lib_name, method_name, &[x]) {
                //                 Ok(Expression::Atom(Atom::Double(result)))
                //             } else {
                //                 Err(format!("Unsupported method call {}.{}", lib_name, method_name))
                //             }
                //         }
                //         _ => Err("Unsupported argument type for method call".to_string()),
                //     }
                // } else {
                //     Err("Method call must have at least one argument".to_string())
                // }
            },
            _ => Err("Unsupported expression type".to_string()),
        }
    }
}
