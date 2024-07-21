use std::fmt::{Debug, Formatter};
use crate::ast::expression::Expression;
use crate::ast::atom::Atom;
use crate::package::lib::Library;

#[derive(Debug)]
pub struct Math;

impl Library for Math {
    fn call_method(&self, func_name: &str, args: Vec<Expression>) -> Result<Expression, String> {

        println!("fuck{:?},{:?}",func_name,args.get(0));
        match func_name {
            "abs" => {
                if let Some(Expression::Atom(Atom::Double(value))) = args.get(0) {
                    Ok(Expression::Atom(Atom::Double(value.abs())))
                } else {
                    Err("Invalid argument for math.abs".to_string())
                }
            }

            _ => Err(format!("Function {} not found in math library", func_name)),
        }
    }
}