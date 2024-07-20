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
        map(parse_function, Expression::from),
        parse_binary_operation,
        parse_tuple,
        parse_function_call, // Add function call parsing here
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
    pub fn run_ast(&self, code: &str) -> Result<HashMap<String, Expression>, String> {
        let (remaining, statements) = many0(preceded(multispace0, parse_statement))(code)
            .map_err(|e| format!("Parse error: {:?}", e))?;

        println!("Parsed statements: {:?}", statements);

        if statements.is_empty() {
            return Err("No statements parsed".to_string());
        }

        let mut local_context = Context::new();

        // First pass: collect function definitions
        for statement in &statements {
            if let Statement::FunctionDefinition(func) = statement {

                println!("------{:?},{:?}",func.name.clone(), func.clone());
               // self.set_function(func.name.clone(), func.clone());

                local_context.set_function(func.name.clone(), func.clone());
            }
        }
        println!("fuck===={:?}", self.get_function("fun"));

        // Second pass: execute statements
        for statement in statements {
            match statement {
                Statement::VariableDeclaration(name, expr) => {
                    let value = expr.evaluate(&local_context)?;
                    local_context.set_variable(name, value);
                }
                Statement::Assignment(name, expr) => {
                    let value = expr.evaluate(&local_context)?;
                    local_context.set_variable(name, value);
                }
                Statement::TupleAssignment(vars, expr) => {
                    let tuple = expr.evaluate(&local_context)?;
                    if let Expression::Tuple(elements) = tuple {
                        if vars.len() == elements.len() {
                            for (var, element) in vars.into_iter().zip(elements) {
                                local_context.set_variable(var, element);
                            }
                        } else {
                            return Err("Tuple assignment mismatch".to_string());
                        }
                    } else {
                        return Err("Expected tuple expression".to_string());
                    }
                }
                Statement::FunctionDefinition(_) => {
                    // Skip function definitions in this pass
                }
                Statement::FunctionCall(name, args) => {
                    let func = local_context.get_function(&name)
                        .ok_or_else(|| format!("Function '{}' not found", name))?;
                    let evaluated_args: Result<Vec<Expression>, String> = args.iter()
                        .map(|arg| arg.evaluate(&local_context))
                        .collect();
                    let result = func.call(evaluated_args?, &local_context)?;

                    if let Expression::Tuple(results) = result {
                        let vars: Vec<String> = args.iter().filter_map(|arg| {
                            if let Expression::Atom(Atom::Variable(var)) = arg {
                                Some(var.clone())
                            } else {
                                None
                            }
                        }).collect();

                        if vars.len() == results.len() {
                            for (var, res) in vars.into_iter().zip(results) {
                                local_context.set_variable(var, res);
                            }
                        }
                    }
                }
            }
        }

        // Create a new HashMap to return the variables
        let variables = local_context.variables.borrow().clone();
        Ok(variables)
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
                let evaluated_exprs: Result<Vec<Expression>, String> = exprs.iter()
                    .map(|e| e.evaluate(context))
                    .collect();
                Ok(Expression::Tuple(evaluated_exprs?))
            }
            Expression::FunctionCall(name, args) => {
                let func = context.get_function(name)
                    .ok_or_else(|| format!("Function '{}' not found", name))?;
                let evaluated_args: Result<Vec<Expression>, String> = args.iter()
                    .map(|arg| arg.evaluate(context))
                    .collect();
                func.call(evaluated_args?, context)
            }
            _ => Err("Unsupported expression type".to_string()),
        }
    }
}

impl Function {
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

        for statement in &self.body.statements {
            match statement {
                Statement::VariableDeclaration(name, expr) => {
                    let value = expr.evaluate(&local_context)?;
                    local_context.set_variable(name.clone(), value);
                }
                _ => {}
            }
        }

        self.body.return_expr.evaluate(&local_context)
    }
}

fn main() {

    let code = "fun(x, y) =>\n    a = x + y\n    b = x - y\n    [a,b]\n[m,n]=fun(8,5)";
    let code = r#"
fun(x, y) =>
    a = x + y
    b = x - y
    [a, b]
[m, n] = fun(8, 5)
"#;

    let context = Context::new();
    let result = context.run_ast(code);
    println!("Result: {:?}", result);
    match result {
        Ok(vars) => {
            println!("Variables: {:?}", vars);
            if let Some(m) = vars.get("m") {
                println!("m: {:?}", m);
            }
            if let Some(n) = vars.get("n") {
                println!("n: {:?}", n);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
