use crate::ast::expression::Expression;
use crate::ast::function::Function;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(String, Expression),
    Assignment(String, Expression),
    TupleAssignment(Vec<String>, Expression),
    FunctionDefinition(Function),
    FunctionCall(String, Vec<Expression>), // Add this variant
}
