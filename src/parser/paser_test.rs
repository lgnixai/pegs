// #[cfg(test)]
// mod tests {
//     use super::*;
//     use super::nom::combinator::all_consuming;
//
//     #[test]
//     fn test_parse_identifier() {
//         let result = parse_identifier("foo");
//         assert_eq!(result, Ok(("", "foo".to_string())));
//     }
//
//     #[test]
//     fn test_parse_atom() {
//         let result = parse_atom("123");
//         assert_eq!(result, Ok(("", Expression::Atom(Atom::Integer(123)))));
//     }
//
//     #[test]
//     fn test_parse_expression() {
//         let result = parse_expression("foo + 1");
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn test_context_variable() {
//         let context = Context::new();
//         context.set_variable("x".to_string(), Expression::Atom(Atom::Integer(42)));
//         let value = context.get_variable("x").unwrap();
//         assert_eq!(value, Expression::Atom(Atom::Integer(42)));
//     }
//
//     #[test]
//     fn test_expression_evaluation() {
//         let context = Context::new();
//         context.set_variable("x".to_string(), Expression::Atom(Atom::Integer(42)));
//         let expr = Expression::Atom(Atom::Variable("x".to_string()));
//         let value = expr.evaluate(&context).unwrap();
//         assert_eq!(value, Expression::Atom(Atom::Integer(42)));
//     }
//
//     #[test]
//     fn test_function_call() {
//         let context = Context::new();
//         let func = Function {
//             name: "add".to_string(),
//             parameters: vec![
//                 Parameter { name: "a".to_string(), default_value: None },
//                 Parameter { name: "b".to_string(), default_value: None },
//             ],
//             body: Block {
//                 statements: vec![],
//                 return_expr: Expression::BinaryOperation(
//                     BinaryOperation::Plus,
//                     Box::new(Expression::Atom(Atom::Variable("a".to_string()))),
//                     Box::new(Expression::Atom(Atom::Variable("b".to_string()))),
//                 ),
//             },
//         };
//         context.set_function("add".to_string(), func);
//         let result = context.run_ast("x = add(1, 2)").unwrap();
//         assert_eq!(result.get("x").unwrap(), &Expression::Atom(Atom::Integer(3)));
//     }
// }
