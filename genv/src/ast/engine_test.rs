#[cfg(test)]
mod tests {
    use crate::ast::engine::Engine;
    use crate::ast::paser::{BinOp, Expr};

    #[test]
    fn test_evaluate() {
        let mut engine = Engine::new();
        let exprs = vec![
            Expr::Assignment("a".to_string(), Box::new(Expr::Number(1))),
            Expr::Assignment("b".to_string(), Box::new(Expr::BinaryOp(
                Box::new(Expr::Variable("a".to_string())),
                BinOp::Add,
                Box::new(Expr::Number(2)),
            ))),
        ];

        engine.evaluate(exprs);
        assert_eq!(engine.get_variable("b"), Some(3));
    }
}
