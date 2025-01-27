// use std::cell::RefCell;
// use std::collections::HashMap;
// use nom::character::complete::multispace0;
// use nom::multi::many0;
// use nom::sequence::preceded;
// use crate::ast::expression::Expression;
// use crate::ast::atom::Atom;
// use crate::ast::function::Function;
// use crate::ast::stmt::Statement;
// use crate::parser::parser::parse_statement;
//
// #[derive(Debug, Clone)]
// pub struct Context {
//     pub(crate) variables: RefCell<HashMap<String, Expression>>,
//     functions: RefCell<HashMap<String, Function>>,
// }
//
// impl Context {
//     pub fn new() -> Self {
//         Context {
//             variables: RefCell::new(HashMap::new()),
//             functions: RefCell::new(HashMap::new()),
//         }
//     }
//
//     pub fn set_variable(&self, name: String, value: Expression) {
//         self.variables.borrow_mut().insert(name, value);
//     }
//
//     pub fn get_variable(&self, name: &str) -> Option<Expression> {
//         self.variables.borrow().get(name).cloned()
//     }
//
//     pub fn set_function(&self, name: String, function: Function) {
//         self.functions.borrow_mut().insert(name, function);
//     }
//
//     pub fn get_function(&self, name: &str) -> Option<Function> {
//         self.functions.borrow().get(name).cloned()
//     }
//     pub fn run_ast(&self, code: &str) -> Result<HashMap<String, Expression>, String> {
//         let (remaining, statements) = many0(preceded(multispace0, parse_statement))(code)
//             .map_err(|e| format!("Parse error: {:?}", e))?;
//
//         println!("Parsed statements: {:?}", statements);
//
//         if statements.is_empty() {
//             return Err("No statements parsed".to_string());
//         }
//
//         let mut local_context = Context::new();
//
//         // First pass: collect function definitions
//         for statement in &statements {
//             if let Statement::FunctionDefinition(func) = statement {
//
//                 println!("------{:?},{:?}",func.name.clone(), func.clone());
//                 // self.set_function(func.name.clone(), func.clone());
//
//                 local_context.set_function(func.name.clone(), func.clone());
//             }
//         }
//         println!("fuck===={:?}", self.get_function("fun"));
//
//         // Second pass: execute statements
//         for statement in statements {
//             match statement {
//                 Statement::VariableDeclaration(name, expr) => {
//                     let value = expr.evaluate(&local_context)?;
//                     local_context.set_variable(name, value);
//                 }
//                 Statement::Assignment(name, expr) => {
//                     let value = expr.evaluate(&local_context)?;
//                     local_context.set_variable(name, value);
//                 }
//                 Statement::TupleAssignment(vars, expr) => {
//                     let tuple = expr.evaluate(&local_context)?;
//                     if let Expression::Tuple(elements) = tuple {
//                         if vars.len() == elements.len() {
//                             for (var, element) in vars.into_iter().zip(elements) {
//                                 local_context.set_variable(var, element);
//                             }
//                         } else {
//                             return Err("Tuple assignment mismatch".to_string());
//                         }
//                     } else {
//                         return Err("Expected tuple expression".to_string());
//                     }
//                 }
//                 Statement::FunctionDefinition(_) => {
//                     // Skip function definitions in this pass
//                 }
//                 Statement::FunctionCall(name, args) => {
//                     let func = local_context.get_function(&name)
//                         .ok_or_else(|| format!("Function '{}' not found", name))?;
//                     let evaluated_args: Result<Vec<Expression>, String> = args.iter()
//                         .map(|arg| arg.evaluate(&local_context))
//                         .collect();
//                     let result = func.call(evaluated_args?, &local_context)?;
//
//                     if let Expression::Tuple(results) = result {
//                         let vars: Vec<String> = args.iter().filter_map(|arg| {
//                             if let Expression::Atom(Atom::Variable(var)) = arg {
//                                 Some(var.clone())
//                             } else {
//                                 None
//                             }
//                         }).collect();
//
//                         if vars.len() == results.len() {
//                             for (var, res) in vars.into_iter().zip(results) {
//                                 local_context.set_variable(var, res);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//
//         // Create a new HashMap to return the variables
//         let variables = local_context.variables.borrow().clone();
//         Ok(variables)
//     }
// }