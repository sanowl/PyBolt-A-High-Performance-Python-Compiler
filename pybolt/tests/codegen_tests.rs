#[cfg(test)]
mod codegen_tests {
    use crate::codegen::codegen::CodeGenerator;
    use crate::ir::ir::IRGenerator;
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_codegen() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        ir_generator.generate(&parser).unwrap();

        let mut code_generator = CodeGenerator::new();
        assert!(code_generator.generate(&ir_generator).is_ok());
    }
}
