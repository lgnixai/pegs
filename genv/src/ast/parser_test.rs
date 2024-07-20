#[cfg(test)]
mod tests {
    use crate::ast::lexer::Token;
    use crate::ast::paser::{BinOp, Expr, parse};

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Number(1),
            Token::Semicolon,
            Token::Identifier("b".to_string()),
            Token::Assign,
            Token::Identifier("a".to_string()),
            Token::Plus,
            Token::Number(2),
            Token::Semicolon,
        ];

        let expected = vec![
            Expr::Assignment("a".to_string(), Box::new(Expr::Number(1))),
            Expr::Assignment("b".to_string(), Box::new(Expr::BinaryOp(
                Box::new(Expr::Variable("a".to_string())),
                BinOp::Add,
                Box::new(Expr::Number(2)),
            ))),
        ];

        let result = parse(&tokens).unwrap();
        assert_eq!(result, expected);
    }
}
