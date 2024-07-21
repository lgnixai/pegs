
use crate::ast::expression::Expression;
use crate::ast::stmt::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expr: Expression,
}
