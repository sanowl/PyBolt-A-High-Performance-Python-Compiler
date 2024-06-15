#[cfg(test)]
mod parser_tests {
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_parser() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        assert!(parser.parse().is_ok());
    }
}
