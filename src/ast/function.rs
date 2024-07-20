use crate::ast::block::Block;
use crate::ast::expression::Expression;
use crate::ast::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}
