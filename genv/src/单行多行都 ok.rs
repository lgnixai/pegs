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

// fn parse_function_body(input: &str) -> IResult<&str, Block> {
//     preceded(
//         delimited(space0, tag("=>"), space0),
//         parse_block,
//     )(input)
// }

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
    //println!("{:?},{:?}",name,input);
    let (input, parameters) = parse_parameter_list(input)?;

    //println!("{:?},{:?}",parameters,input);

    let (input, body) = parse_function_body(input)?;

    println!("body::{:?},{:?}",body,input);

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
    variables: RefCell<HashMap<String, Expression>>,
    functions: RefCell<HashMap<String, Function>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: RefCell::new(HashMap::new()),
            functions: RefCell::new(HashMap::new()),
        }
    }

    pub fn set_variable(&self, name: String, value: Expression) {
        self.variables.borrow_mut().insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<Expression> {
        self.variables.borrow().get(name).cloned()
    }

    pub fn set_function(&self, name: String, function: Function) {
        self.functions.borrow_mut().insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        self.functions.borrow().get(name).cloned()
    }
}

impl Expression {
    pub fn evaluate(&self, context: &Context) -> Result<Expression, String> {
        match self {
            Expression::Atom(Atom::Variable(var_name)) => {
                if let Some(value) = context.get_variable(var_name) {
                    value.evaluate(context)
                } else {
                    Err(format!("Undefined variable: {}", var_name))
                }
            },
            Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
            Expression::BinaryOperation(op, left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;

                println!("Evaluating binary operation: {:?} {:?} {:?}", left_val, op, right_val); // Debugging line

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
                context.set_function(func.name.clone(), *(*func).clone());
                Ok(Expression::Function(func.clone()))
            }
            Expression::Tuple(exprs) => {
                println!("tuple {:?}",exprs);
                let evaluated_exprs: Result<Vec<Expression>, String> = exprs.iter()
                    .map(|e| e.evaluate(context))
                    .collect();
                Ok(Expression::Tuple(evaluated_exprs?))
            }
            _ => Err("Unsupported expression type".to_string()),
        }
    }
}

impl Function {
    // pub fn call(&self, args: Vec<Expression>, context: &Context) -> Result<Expression, String> {
    //     if args.len() != self.parameters.len() {
    //         return Err("Argument count mismatch".to_string());
    //     }
    //
    //     let mut local_context = Context::new();
    //     local_context.variables.borrow_mut().extend(context.variables.borrow().clone());
    //
    //     for (param, arg) in self.parameters.iter().zip(args) {
    //         let evaluated_arg = arg.evaluate(&local_context)?;
    //         local_context.set_variable(param.name.clone(), evaluated_arg);
    //
    //         //println!("{:?},{:?}",param.name.clone(), evaluated_arg);
    //     }
    //
    //     self.body.return_expr.evaluate(&local_context)
    // }

    pub fn call(&self, args: Vec<Expression>, context: &Context) -> Result<Expression, String> {
        if args.len() != self.parameters.len() {
            return Err("Argument count mismatch".to_string());
        }

        let mut local_context = Context::new();
        local_context.variables.borrow_mut().extend(context.variables.borrow().clone());

        for (param, arg) in self.parameters.iter().zip(args) {
            let evaluated_arg = arg.evaluate(&local_context)?;
            local_context.set_variable(param.name.clone(), evaluated_arg);
        }
        println!("rrrrr{:?}",3333);

        for statement in &self.body.statements {
            match statement {
                Statement::VariableDeclaration(name, expr) => {
                    let value = expr.evaluate(&local_context)?;
                    local_context.set_variable(name.clone(), value);
                }
            }
        }
        let result = self.body.return_expr.evaluate(&local_context)?;

        println!("rrrrr{:?}",result);
        Ok(result)
    }
}

fn main() {
    // Define a single-line function
    let single_line_function_code = "add(a, b) => a + b";
    let (remaining, single_line_function) = parse_function(single_line_function_code).expect("Failed to parse function");

    // Define a multi-line function
    // let multi_line_function_code = r#"multiply(x, y) =>
    // z = x * y
    // z
    // "#;
    let multi_line_function_code = "fun(x, y) =>\n    a = x + y\n    b = x - y\n    [a,b]";
    let input_script = r#"
fun(x, y)=>
    a = x+y
    b=x-y
    [a,b]
"#;

    let (remaining, multi_line_function) = parse_function(multi_line_function_code).expect("Failed to parse function");


    println!("{:?}",multi_line_function);
    // Create a context and set the functions
    let context = Context::new();
    context.set_function(single_line_function.name.clone(), single_line_function);
    context.set_function(multi_line_function.name.clone(), multi_line_function);

    // Test single-line function
    {
        let add_function = context.get_function("add").expect("Function 'add' not found");
        let add_args = vec![
            Expression::Atom(Atom::Integer(5)),
            Expression::Atom(Atom::Integer(3))
        ];
        let result = add_function.call(add_args, &context).expect("Function call failed");
        println!("Result of single-line function call: {:?}", result);
    }

    // Test multi-line function
    {
        let multiply_function = context.get_function("fun").expect("Function 'multiply' not found");
        let multiply_args = vec![
            Expression::Atom(Atom::Integer(14)),
            Expression::Atom(Atom::Integer(7))
        ];
        let result = multiply_function.call(multiply_args, &context).expect("Function call failed");
        println!("Result of multi-line function call: {:?}", result);
    }
}
