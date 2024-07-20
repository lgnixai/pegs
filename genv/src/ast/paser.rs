use crate::ast::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Variable(String),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    Assignment(String, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expr>, String> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        let expr = parse_expr(tokens, &mut pos)?;
        exprs.push(expr);

        if let Some(Token::Semicolon) = tokens.get(pos) {
            pos += 1;
        } else {
            return Err("Expected semicolon".to_string());
        }
    }

    Ok(exprs)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let lhs = parse_primary(tokens, pos)?;
    if let Some(op) = parse_binop(tokens, pos) {
        let rhs = parse_primary(tokens, pos)?;
        return Ok(Expr::BinaryOp(Box::new(lhs), op,
                                 Box::new(rhs)));
    }
    Ok(lhs)
}

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if let Some(token) = tokens.get(*pos) {
        match token {
            Token::Number(n) => {
                *pos += 1;
                return Ok(Expr::Number(*n));
            }
            Token::Identifier(name) => {
                *pos += 1;
                if let Some(Token::Assign) = tokens.get(*pos) {
                    *pos += 1;
                    let expr = parse_expr(tokens, pos)?;
                    return Ok(Expr::Assignment(name.clone(), Box::new(expr)));
                }
                return Ok(Expr::Variable(name.clone()));
            }
            _ => return Err("Expected a number or identifier".to_string()),
        }
    }
    Err("Unexpected end of input".to_string())
}

fn parse_binop(tokens: &[Token], pos: &mut usize) -> Option<BinOp> {
    if let Some(token) = tokens.get(*pos) {
        match token {
            Token::Plus => {
                *pos += 1;
                Some(BinOp::Add)
            }
            Token::Minus => {
                *pos += 1;
                Some(BinOp::Sub)
            }
            Token::Multiply => {
                *pos += 1;
                Some(BinOp::Mul)
            }
            Token::Divide => {
                *pos += 1;
                Some(BinOp::Div)
            }
            _ => None,
        }
    } else {
        None
    }
}
