#[cfg(test)]
mod tests {
    use crate::ast::lexer::{Token, tokenize};

    #[test]
    fn test_tokenize() {
        let input = "a = 1 + 2;";
        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Semicolon,
        ];
        let result = tokenize(input).unwrap().1;
        assert_eq!(result, expected);
    }
}
