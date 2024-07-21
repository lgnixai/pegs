
use crate::ast::atom::Atom;
use crate::ast::binaryop::BinaryOperation;
use crate::ast::function::Function;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Expression2 {
//     BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
//     Atom(Atom),
//     Function(Box<Function>),
//     Tuple(Vec<Expression>),
//     FunctionCall(String, Vec<Expression>),
//     MethodCall(String, String, Vec<Expression>),
//
// }
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    Atom(Atom),
    Function(Box<Function>),
    MethodCall(String, String, Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
    Tuple(Vec<Expression>),
}

// #[derive(PartialEq)]
impl From<Function> for Expression {
    fn from(func: Function) -> Self {
        Expression::Function(Box::new(func))
    }
}