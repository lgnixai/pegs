
use crate::ast::atom::Atom;
use crate::ast::binaryop::BinaryOperation;
use crate::ast::function::Function;

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    Atom(Atom),
    Function(Box<Function>),
    Tuple(Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
}


impl From<Function> for Expression {
    fn from(func: Function) -> Self {
        Expression::Function(Box::new(func))
    }
}