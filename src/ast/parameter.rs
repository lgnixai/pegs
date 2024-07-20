use crate::ast::expression::Expression;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expression>,
}
