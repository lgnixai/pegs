use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::map,
    multi::many0,
    sequence::{delimited, pair, preceded, terminated}
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Semicolon,
}

fn identifier(input: &str) -> IResult<&str, Token> {
    map(alpha1, |s: &str| Token::Identifier(s.to_string()))(input)
}

fn number(input: &str) -> IResult<&str, Token> {
    map(digit1, |s: &str| Token::Number(s.parse().unwrap()))(input)
}

fn plus(input: &str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Plus)(input)
}

fn minus(input: &str) -> IResult<&str, Token> {
    map(tag("-"), |_| Token::Minus)(input)
}

fn multiply(input: &str) -> IResult<&str, Token> {
    map(tag("*"), |_| Token::Multiply)(input)
}

fn divide(input: &str) -> IResult<&str, Token> {
    map(tag("/"), |_| Token::Divide)(input)
}

fn assign(input: &str) -> IResult<&str, Token> {
    map(tag("="), |_| Token::Assign)(input)
}

fn semicolon(input: &str) -> IResult<&str, Token> {
    map(tag(";"), |_| Token::Semicolon)(input)
}

fn token(input: &str) -> IResult<&str, Token> {
    preceded(multispace0, alt((identifier, number, plus, minus, multiply, divide, assign, semicolon)))(input)
}

pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    many0(token)(input)
}
