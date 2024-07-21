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
    Tuple(Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub return_expr: Expression,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(String, Expression),
    Assignment(String, Expression),
    TupleAssignment(Vec<String>, Expression),
    FunctionDefinition(Function),
    FunctionCall(String, Vec<Expression>), // Add this variant
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



fn parse_statement(input: &str) -> IResult<&str, Statement> {
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

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_binary_operation,
        parse_tuple,
        parse_function_call,
        parse_atom,
    ))(input)
}

pub trait BuiltinLibrary {
    fn call(&self, func_name: &str, args: Vec<Expression>) -> Result<Expression, String>;
}

pub struct MathLibrary;

impl BuiltinLibrary for MathLibrary {
    fn call(&self, func_name: &str, args: Vec<Expression>) -> Result<Expression, String> {
        match func_name {
            "math.abs" => {
                if let Some(Expression::Atom(Atom::Integer(value))) = args.get(0) {
                    Ok(Expression::Atom(Atom::Integer(value.abs())))
                } else {
                    Err("Invalid argument for math.abs".to_string())
                }
            }
            _ => Err(format!("Function {} not found in math library", func_name)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    variables: RefCell<HashMap<String, Expression>>,
    functions: RefCell<HashMap<String, Function>>,
    builtins: RefCell<HashMap<String, Box<dyn BuiltinLibrary>>>,  // Add this field
}

impl Context {
    pub fn new() -> Self {
        let mut context = Context {
            variables: RefCell::new(HashMap::new()),
            functions: RefCell::new(HashMap::new()),
            builtins: RefCell::new(HashMap::new()),  // Initialize builtins
        };

        // Register built-in libraries
        context.register_builtin("math", Box::new(MathLibrary));

        context
    }

    pub fn register_builtin(&mut self, name: &str, library: Box<dyn BuiltinLibrary>) {
        self.builtins.borrow_mut().insert(name.to_string(), library);
    }

    pub fn get_builtin(&self, name: &str) -> Option<Box<dyn BuiltinLibrary>> {
        self.builtins.borrow().get(name).cloned()
    }
}

impl Expression {
    pub fn evaluate(&self, context: &Context) -> Result<Expression, String> {
        match self {
            Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
            Expression::BinaryOperation(op, left, right) => {
                let left_value = left.evaluate(context)?;
                let right_value = right.evaluate(context)?;
                match (op, left_value, right_value) {
                    (BinaryOperation::Plus, Expression::Atom(Atom::Integer(a)), Expression::Atom(Atom::Integer(b))) => Ok(Expression::Atom(Atom::Integer(a + b))),
                    (BinaryOperation::Minus, Expression::Atom(Atom::Integer(a)), Expression::Atom(Atom::Integer(b))) => Ok(Expression::Atom(Atom::Integer(a - b))),
                    (BinaryOperation::Times, Expression::Atom(Atom::Integer(a)), Expression::Atom(Atom::Integer(b))) => Ok(Expression::Atom(Atom::Integer(a * b))),
                    (BinaryOperation::Divide, Expression::Atom(Atom::Integer(a)), Expression::Atom(Atom::Integer(b))) => Ok(Expression::Atom(Atom::Integer(a / b))),
                    (BinaryOperation::Equal, left, right) => Ok(Expression::Atom(Atom::Boolean(left == right))),
                    (BinaryOperation::NotEqual, left, right) => Ok(Expression::Atom(Atom::Boolean(left != right))),
                    _ => Err("Unsupported operation".to_string()),
                }
            }
            Expression::Tuple(exprs) => {
                let evaluated_exprs: Result<Vec<Expression>, String> = exprs.iter().map(|e| e.evaluate(context)).collect();
                Ok(Expression::Tuple(evaluated_exprs?))
            }
            Expression::Function(func) => {
                Ok(Expression::Function(func.clone()))
            }
            Expression::FunctionCall(name, args) => {
                if let Some(library_name) = name.split('.').next() {
                    if let Some(library) = context.get_builtin(library_name) {
                        return library.call(name, args.clone());
                    }
                }

                // existing function call handling
                if let Some(func) = context.get_function(name) {
                    let evaluated_args: Result<Vec<Expression>, String> = args.iter()
                        .map(|arg| arg.evaluate(context))
                        .collect();
                    return func.call(evaluated_args?, context);
                }

                Err(format!("Function '{}' not found", name))
            }
        }
    }
}

impl Function {
    pub fn call(&self, args: Vec<Expression>, context: &Context) -> Result<Expression, String> {
        if args.len() != self.parameters.len() {
            return Err("Incorrect number of arguments".to_string());
        }

        let mut local_context = context.clone();
        for (param, arg) in self.parameters.iter().zip(args) {
            local_context.variables.borrow_mut().insert(param.name.clone(), arg);
        }

        for statement in &self.body.statements {
            statement.execute(&local_context)?;
        }

        self.body.return_expr.evaluate(&local_context)
    }
}

impl Statement {
    pub fn execute(&self, context: &Context) -> Result<(), String> {
        match self {
            Statement::VariableDeclaration(name, expr) => {
                let value = expr.evaluate(context)?;
                context.variables.borrow_mut().insert(name.clone(), value);
                Ok(())
            }
            Statement::Assignment(name, expr) => {
                let value = expr.evaluate(context)?;
                if context.variables.borrow().contains_key(name) {
                    context.variables.borrow_mut().insert(name.clone(), value);
                    Ok(())
                } else {
                    Err(format!("Variable '{}' not found", name))
                }
            }
            Statement::TupleAssignment(names, expr) => {
                let value = expr.evaluate(context)?;
                if let Expression::Tuple(values) = value {
                    if names.len() == values.len() {
                        for (name, value) in names.iter().zip(values) {
                            context.variables.borrow_mut().insert(name.clone(), value);
                        }
                        Ok(())
                    } else {
                        Err("Tuple lengths do not match".to_string())
                    }
                } else {
                    Err("Right-hand side is not a tuple".to_string())
                }
            }
            Statement::FunctionDefinition(func) => {
                context.functions.borrow_mut().insert(func.name.clone(), func.clone());
                Ok(())
            }
            Statement::FunctionCall(name, args) => {
                if let Some(func) = context.functions.borrow().get(name) {
                    let evaluated_args: Result<Vec<Expression>, String> = args.iter()
                        .map(|arg| arg.evaluate(context))
                        .collect();
                    func.call(evaluated_args?, context)?;
                    Ok(())
                } else {
                    Err(format!("Function '{}' not found", name))
                }
            }
        }
    }
}

impl Context {
    pub fn get_variable(&self, name: &str) -> Option<Expression> {
        self.variables.borrow().get(name).cloned()
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        self.functions.borrow().get(name).cloned()
    }
}

// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_abs() {
        let code = "x = math.abs(-1)";
        let context = Context::new();
        let ast = parse_statement(code).unwrap().1;
        ast.execute(&context).unwrap();

        if let Some(Expression::Atom(Atom::Integer(x))) = context.get_variable("x") {
            assert_eq!(x, 1);
        } else {
            panic!("Variable 'x' not found or incorrect type");
        }
    }
}

fn main() {
    let code = "x = math.abs(-1)";
    let context = Context::new();
    let ast = parse_statement(code).unwrap().1;
    ast.execute(&context).unwrap();

    if let Some(Expression::Atom(Atom::Integer(x))) = context.get_variable("x") {
        println!("x: {:?}", x); // Should print: x: 1
    } else {
        println!("Variable 'x' not found or incorrect type");
    }
}
