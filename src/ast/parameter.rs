use crate::ast::expression::Expression;

#[derive(Debug, Clone,PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expression>,
}
