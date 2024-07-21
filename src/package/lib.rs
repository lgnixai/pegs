use std::fmt;
use crate::ast::expression::Expression;

pub trait Library: fmt::Debug {
    fn call_method(&self, name: &str, args: Vec<Expression>) -> Result<Expression, String>;
}