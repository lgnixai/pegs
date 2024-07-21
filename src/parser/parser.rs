use std::cell::RefCell;
use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, i64, space0, char};
use nom::combinator::{map, opt};
use nom::multi::{many0, separated_list0};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use crate::ast::atom::Atom;
use crate::ast::binaryop::BinaryOperation;
use crate::ast::block::Block;
use crate::ast::expression::Expression;
use crate::ast::function::Function;
use crate::ast::parameter::Parameter;
use crate::ast::stmt::Statement;


fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(input)
}

fn parse_parameter(input: &str) -> IResult<&str, Parameter> {
    let (input, name) = parse_identifier(input)?;
    let (input, default_value) = opt(preceded(tag("="), parse_expression))(input)?;
    Ok((input, Parameter { name, default_value }))
}

fn parse_parameter_list(input: &str) -> IResult<&str, Vec<Parameter>> {
    delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_parameter,
        ),
        tag(")")
    )(input)
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    let (input, statements) = many0(preceded(multispace0, parse_statement))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, return_expr) = parse_expression(input)?;
    Ok((input, Block { statements, return_expr }))
}

fn parse_single_line_body(input: &str) -> IResult<&str, Block> {
    let (input, return_expr) = parse_expression(input)?;
    Ok((input, Block { statements: Vec::new(), return_expr }))
}

fn parse_function_body(input: &str) -> IResult<&str, Block> {
    alt((
        preceded(
            delimited(space0, tag("=>"), space0),
            parse_single_line_body,
        ),
        delimited(
            delimited(space0, tag("=>"), space0),
            parse_block,
            multispace0
        )
    ))(input)
}

fn parse_variable_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = parse_expression(input)?;
    Ok((input, Statement::VariableDeclaration(name, expr)))
}

fn parse_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = parse_expression(input)?;
    Ok((input, Statement::Assignment(name, expr)))
}

fn parse_tuple_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, vars) = delimited(
        tag("["),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_identifier
        ),
        tag("]")
    )(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = parse_expression(input)?;
    Ok((input, Statement::TupleAssignment(vars, expr)))
}


fn parse_function_definition(input: &str) -> IResult<&str, Statement> {
    let (input, function) = parse_function(input)?;
    Ok((input, Statement::FunctionDefinition(function)))
}


fn parse_function_call_statement(input: &str) -> IResult<&str, Statement> {
    let (input, func_name) = parse_identifier(input)?;
    let (input, args) = delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_expression
        ),
        tag(")")
    )(input)?;
    Ok((input, Statement::FunctionCall(func_name, args)))
}




pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_variable_declaration,
        parse_assignment,
        parse_tuple_assignment,
        parse_function_definition,
        parse_function_call_statement, // Ensure function calls are parsed

    ))(input)
}

fn parse_function(input: &str) -> IResult<&str, Function> {
    let (input, name) = parse_identifier(input)?;
    let (input, parameters) = parse_parameter_list(input)?;
    let (input, body) = parse_function_body(input)?;
    Ok((input, Function { name, parameters, body }))
}

fn parse_string(input: &str) -> IResult<&str, Atom> {
    let (input, s) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    Ok((input, Atom::String(s.to_string())))
}

fn parse_boolean(input: &str) -> IResult<&str, Atom> {
    alt((
        map(tag("true"), |_| Atom::Boolean(true)),
        map(tag("false"), |_| Atom::Boolean(false)),
    ))(input)
}

fn parse_double(input: &str) -> IResult<&str, Atom> {
    map(double, Atom::Double)(input)
}

fn parse_integer(input: &str) -> IResult<&str, Atom> {
    map(i64, Atom::Integer)(input)
}

fn parse_variable(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |var: &str| Atom::Variable(var.to_string()))(input)
}

fn parse_tuple(input: &str) -> IResult<&str, Expression> {
    let (input, exprs) = delimited(
        tag("["),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_expression
        ),
        tag("]")
    )(input)?;
    Ok((input, Expression::Tuple(exprs)))
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

fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    let (input, func_name) = parse_identifier(input)?;
    let (input, args) = delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_expression
        ),
        tag(")")
    )(input)?;
    Ok((input, Expression::FunctionCall(func_name, args)))
}


fn parse_import(input: &str) -> IResult<&str, Expression> {
    let (input, _) = tag("import")(input)?;
    let (input, _) = space0(input)?;
    let (input, library_name) = parse_identifier(input)?;
    Ok((input, Expression::MethodCall("import".to_string(), library_name, Vec::new())))
}

fn parse_method_call(input: &str) -> IResult<&str, Expression> {
    let (input, obj_name) = parse_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, method_name) = delimited(tag("."), parse_identifier, space0)(input)?;
    let (input, args) = delimited(
        tag("("),
        separated_list0(
            delimited(space0, tag(","), space0),
            parse_expression
        ),
        tag(")")
    )(input)?;
    Ok((input, Expression::MethodCall(obj_name, method_name, args)))
}

pub(crate) fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_function, Expression::from),
        parse_binary_operation,
        parse_tuple,
        parse_import,
        parse_method_call,
        parse_function_call, // Add function call parsing here
        parse_atom,
    ))(input)
}