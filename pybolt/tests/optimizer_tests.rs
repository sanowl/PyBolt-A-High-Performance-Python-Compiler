#[cfg(test)]
mod optimizer_tests {
    use crate::optimizer::optimizer::Optimizer;
    use crate::ir::ir::IRGenerator;
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_optimization() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        ir_generator.generate(&parser).unwrap();

        let mut optimizer = Optimizer::new();
        assert!(optimizer.optimize(&ir_generator).is_ok());
    }
}
