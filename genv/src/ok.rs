use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, i64, multispace0};
use nom::combinator::map;
use nom::IResult;
use nom::number::complete::double;
use nom::sequence::{delimited, tuple};
use serde::{Deserialize, Serialize};
use crate::ast::vm::VM;

mod ast;
mod ok;

fn ok() {
    let mut vm = VM::new();
    let script = "a = 1; b = a + 2;";
    vm.execute(script);
    println!("Value of b: {}", vm.get_variable("b").unwrap());
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum Atom {
    String(String),
    Variable(String),
    Boolean(bool),
    Integer(i64),
    Double(f64),

}
#[derive(Debug)]
pub enum Expression {

    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    Atom(Atom),
    Comment(String),
    SortingHat,
}
pub fn parse_binary_operator(input: &str) -> IResult<&str, BinaryOperation> {
    alt((
        map(char('+'), |_| BinaryOperation::Plus),
        map(char('-'), |_| BinaryOperation::Minus),
        map(char('*'), |_| BinaryOperation::Times),
        map(char('/'), |_| BinaryOperation::Divide),
        map(tag("=="), |_| BinaryOperation::Equal),
        map(tag("!="), |_| BinaryOperation::NotEqual),
    ))(input)
}
fn parse_atom(input: &str) -> IResult<&str, Expression> {
    let parser = alt((
        parse_boolean,
        parse_double,
        parse_integer,
        parse_string,
        parse_variable,
    ));
    map(parser, |atom| atom.into())(input)
}


fn parse_boolean(input: &str) -> IResult<&str, Atom> {
    let parser = alt((tag("true"), tag("false")));
    map(parser, |boolean: &str| Atom::Boolean(boolean == "true"))(input)
}

fn parse_double(input: &str) -> IResult<&str, Atom> {
    if !input.contains(".") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }

    let parser = double;
    map(parser, |float| Atom::Double(float))(input)
}

fn parse_integer(input: &str) -> IResult<&str, Atom> {
    let parser = i64;
    map(parser, |integer| Atom::Integer(integer))(input)
}

fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |var: &str| Atom::Variable(var.to_string()))(input)
}
pub fn parse_binary_operation(input: &str) -> IResult<&str, Expression> {
    let (rest, (left, _, op, _, right)) = tuple((
        parse_atom,
        multispace0,
        parse_binary_operator,
        multispace0,
        parse_atom,
    ))(input)?;

    let expression = Expression::BinaryOperation(op, Box::new(left), Box::new(right));
    Ok((rest, expression))
}
fn main() {
    let input = "true";
    let expected = Atom::Boolean(true);
    let (_, actual) = parse_boolean(input).unwrap();
    println!("{:?}",actual);

}