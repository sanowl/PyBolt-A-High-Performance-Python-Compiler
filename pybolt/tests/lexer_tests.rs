#[cfg(test)]
mod lexer_tests {
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token().unwrap().lexeme, "let");
    }
}
