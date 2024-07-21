use std::cell::RefCell;
use std::collections::HashMap;
use crate::ast::atom::Atom;
use crate::ast::expression::Expression;
use crate::ast::function::Function;

#[derive(Debug,Clone)]
pub struct TA {

    pub(crate) methods: RefCell<HashMap<String, fn(&TA, Vec<Expression>) -> Result<Expression, String>>>,
}

impl TA {
    pub fn new() -> Self {
        let mut methods =RefCell::new(HashMap::new());
        //methods.borrow().insert("change".to_string(), TA::handle_change);

         methods.borrow_mut().insert("change".to_string(), TA::handle_change as fn(&TA, Vec<Expression>) -> Result<Expression, String>);
        //methods.insert("cci".to_string(), TA::handle_cci as fn(&TA, Vec<Expression>) -> Result<Expression, String>);
        TA { methods }
    }

    pub fn handle_change(ta: &TA, args: Vec<Expression>) -> Result<Expression, String> {
        if args.len() == 2 {
            if let (Expression::Atom(Atom::String(input)), Expression::Atom(Atom::Integer(value))) = (&args[0], &args[1]) {
                Ok(Expression::Atom(Atom::Integer(ta.change(input, *value))))
            } else {
                Err("Invalid arguments for ta.change".to_string())
            }
        } else {
            Err("Invalid number of arguments for ta.change".to_string())
        }
    }

    pub fn handle_cci(ta: &TA, args: Vec<Expression>) -> Result<Expression, String> {
        if args.len() == 2 {
            if let (Expression::Atom(Atom::String(input)), Expression::Atom(Atom::Integer(value))) = (&args[0], &args[1]) {
                Ok(Expression::Atom(Atom::Double(ta.cci(input, *value))))
            } else {
                Err("Invalid arguments for ta.cci".to_string())
            }
        } else {
            Err("Invalid number of arguments for ta.cci".to_string())
        }
    }

    fn change(&self, input: &str, value: i64) -> i64 {
        value
    }

    fn cci(&self, input: &str, value: i64) -> f64 {
        value as f64
    }
}

enum Object {
    TA(TA),
}
