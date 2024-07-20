use crate::ast::expression::Expression;

#[derive(Debug, Clone)]
pub enum Atom {
    String(String),
    Variable(String),
    Boolean(bool),
    Integer(i64),
    Double(f64),
}

impl From<Atom> for Expression {
    fn from(atom: Atom) -> Self {
        Expression::Atom(atom)
    }
}
