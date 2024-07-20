 
use std::collections::HashMap;
use crate::ast::paser::{Expr,BinOp};

pub struct Engine {
    variables: HashMap<String, i32>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            variables: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, exprs: Vec<Expr>) {
        for expr in exprs {
            self.eval_expr(expr);
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<i32> {
        self.variables.get(name).cloned()
    }

    fn eval_expr(&mut self, expr: Expr) -> i32 {
        match expr {
            Expr::Number(n) => n,
            Expr::Variable(name) => *self.variables.get(&name).unwrap_or(&0),
            Expr::BinaryOp(lhs, op, rhs) => {
                let lhs_val = self.eval_expr(*lhs);
                let rhs_val = self.eval_expr(*rhs);
                match op {
                    BinOp::Add => lhs_val + rhs_val,
                    BinOp::Sub => lhs_val - rhs_val,
                    BinOp::Mul => lhs_val * rhs_val,
                    BinOp::Div => lhs_val / rhs_val,
                }
            }
            Expr::Assignment(name, expr) => {
                let val = self.eval_expr(*expr);
                self.variables.insert(name, val);
                val
            }
        }
    }
}
