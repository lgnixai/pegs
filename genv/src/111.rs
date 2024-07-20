use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, i64, space0, char, newline};
use nom::combinator::{map, opt};
use nom::multi::{many0, separated_list0};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
}

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

impl From<Function> for Expression {
    fn from(func: Function) -> Self {
        Expression::Function(Box::new(func))
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    Atom(Atom),
    Function(Box<Function>),
    Comment(String),
    Tuple(Vec<Expression>),
    SortingHat,
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    parameters: Vec<Parameter>,
    body: Block,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    name: String,
    default_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
    return_expr: Expression,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(String, Expression),
}

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

fn parse_function_body(input: &str) -> IResult<&str, Block> {
    preceded(
        delimited(space0, tag("=>"), space0),
        parse_block,
    )(input)
}

fn parse_variable_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = parse_expression(input)?;
    Ok((input, Statement::VariableDeclaration(name, expr)))
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_variable_declaration,
        // Add parsers for other statement types if needed.
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
    let (input, left) = parse_atom(input)?;
    let (input, _) = multispace0(input)?;
    let (input, op) = parse_binary_operator(input)?;
    let (input, _) = multispace0(input)?;
    let (input, right) = parse_atom(input)?;
    let expression = Expression::BinaryOperation(op, Box::new(left), Box::new(right));
    Ok((input, expression))
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_function, Expression::from),
        parse_binary_operation,
        parse_tuple,
        parse_atom,
    ))(input)
}

#[derive(Debug, Clone)]
pub struct Context {
    variables: HashMap<String, Expression>,
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Expression) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Expression> {
        self.variables.get(name)
    }

    pub fn set_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}

impl Expression {
    pub fn evaluate(&self, context: &mut Context) -> Result<Expression, String> {
        match self {
            Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
            Expression::BinaryOperation(op, left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (left_val, right_val) {
                    (Expression::Atom(Atom::Integer(l)), Expression::Atom(Atom::Integer(r))) => {
                        match op {
                            BinaryOperation::Plus => Ok(Expression::Atom(Atom::Integer(l + r))),
                            BinaryOperation::Minus => Ok(Expression::Atom(Atom::Integer(l - r))),
                            BinaryOperation::Times => Ok(Expression::Atom(Atom::Integer(l * r))),
                            BinaryOperation::Divide => Ok(Expression::Atom(Atom::Integer(l / r))),
                            _ => Err("Unsupported binary operation".to_string()),
                        }
                    }
                    (Expression::Atom(Atom::Double(l)), Expression::Atom(Atom::Double(r))) => {
                        match op {
                            BinaryOperation::Plus => Ok(Expression::Atom(Atom::Double(l + r))),
                            BinaryOperation::Minus => Ok(Expression::Atom(Atom::Double(l - r))),
                            BinaryOperation::Times => Ok(Expression::Atom(Atom::Double(l * r))),
                            BinaryOperation::Divide => Ok(Expression::Atom(Atom::Double(l / r))),
                            _ => Err("Unsupported binary operation".to_string()),
                        }
                    }
                    _ => Err("Type error in binary operation".to_string()),
                }
            }
            Expression::Function(func) => {
                context.set_function(func.name.clone(), (**func).clone());
                Ok(Expression::Function(func.clone()))
            }
            Expression::Tuple(exprs) => {
                let evaluated_exprs: Result<Vec<Expression>, String> = exprs.iter()
                    .map(|e| e.evaluate(context))
                    .collect();
                Ok(Expression::Tuple(evaluated_exprs?))
            }
            Expression::Atom(Atom::Variable(name)) => {
                if let Some(value) = context.get_variable(name) {
                    Ok(value.clone())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            _ => Err("Unsupported expression".to_string()),
        }
    }
}

impl Statement {
    pub fn execute(&self, context: &mut Context) -> Result<(), String> {
        match self {
            Statement::VariableDeclaration(name, expr) => {
                let value = expr.evaluate(context)?;
                context.set_variable(name.clone(), value);
                Ok(())
            }
        }
    }
}

impl Function {
    pub fn call(&self, args: Vec<Expression>, context: &mut Context) -> Result<Expression, String> {
        if args.len() != self.parameters.len() {
            return Err("Incorrect number of arguments".to_string());
        }

        let mut local_context = context.clone();
        for (param, arg) in self.parameters.iter().zip(args) {
            local_context.set_variable(param.name.clone(), arg);
        }

        for stmt in &self.body.statements {
            stmt.execute(&mut local_context)?;
        }

        self.body.return_expr.evaluate(&mut local_context)
    }
}

fn main() {
    let input_single_line = r#"f(x,y) => x+y
    a=f(2,3)"#;

    match parse_statement(input_single_line) {
        Ok((_, stmt)) => {
            let mut context = Context::new();
            if let Err(e) = stmt.execute(&mut context) {
                println!("Execution error: {:?}", e);
            }
            if let Some(result) = context.get_variable("a") {
                println!("a = {:?}", result);
            } else {
                println!("Variable 'a' not found");
            }
        }
        Err(err) => eprintln!("Error: {:?}", err),
    }


    // Additional tests and usage examples
    let input_multi_line = "fun(x, y) =>\n    a = x + y\n    b = x - y\n    [a,b]";
    let input_script = r#"fun(x, y)=>
    a = x + y
    b = x - y
    [a,b]
"#;

    match parse_variable(input_single_line) {
        Ok((_, expr)) => println!("{:?}", expr),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    let mut context = Context::new();

    let (_, expr) = parse_expression(input_single_line).expect("Failed to parse script");
    println!("{:?}", expr);
    expr.evaluate(&mut context).expect("Failed to evaluate expression");

    if let Some(result) = context.get_variable("a") {
        println!("a = {:?}", result);
    } else {
        println!("Variable 'a' not found");
    }

    match parse_expression(input_multi_line) {
        Ok((_, expr)) => println!("{:?}", expr),
        Err(err) => eprintln!("Error: {:?}", err),
    }
    match parse_expression(input_script) {
        Ok((_, expr)) => println!("{:?}", expr),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
