#[cfg(test)]
mod semantic_tests {
    use crate::semantic::semantic::SemanticAnalyzer;
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_semantic_analysis() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut analyzer = SemanticAnalyzer::new();
        assert!(analyzer.analyze(&parser).is_ok());
    }
}
