pub mod ir;

#[cfg(test)]
mod tests {
    use super::ir::IRGenerator;
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_ir_generation() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        assert!(ir_generator.generate(&parser).is_ok());
    }
}
