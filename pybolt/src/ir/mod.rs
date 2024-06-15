pub mod ir;

#[cfg(test)]
mod tests {
    use super::ir::{IRGenerator, IR};
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    #[test]
    fn test_ir_generation() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&parser).expect("IR generation failed");

        assert_eq!(ir.len(), 1);
        if let IR::VariableDeclaration { name, value } = &ir[0] {
            assert_eq!(name, "x");
            assert_eq!(*value, 42);
        } else {
            panic!("Expected variable declaration IR");
        }
    }

    #[test]
    fn test_ir_generation_with_binary_operation() {
        let input = "let y = 1 + 2;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&parser).expect("IR generation failed");

        assert_eq!(ir.len(), 1);
        if let IR::BinaryOperation { operator, left, right } = &ir[0] {
            assert_eq!(operator, "+");
            match (&**left, &**right) {
                (IR::Immediate { value: left_val, .. }, IR::Immediate { value: right_val, .. }) => {
                    assert_eq!(*left_val, 1);
                    assert_eq!(*right_val, 2);
                }
                _ => panic!("Expected immediate values for binary operation"),
            }
        } else {
            panic!("Expected binary operation IR");
        }
    }
}
