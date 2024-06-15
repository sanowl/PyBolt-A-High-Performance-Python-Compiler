pub mod codegen;

#[cfg(test)]
mod tests {
    use super::codegen::{CodeGenerator, MachineInstruction};
    use crate::ir::ir::{IRGenerator, IR};
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_codegen_variable_declaration() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&parser).unwrap();

        let mut code_generator = CodeGenerator::new();
        let instructions = code_generator.generate(&ir_generator).expect("Code generation failed");

        assert_eq!(instructions.len(), 1);
        if let MachineInstruction::LoadImmediate { dest, value } = &instructions[0] {
            assert_eq!(dest, "x");
            assert_eq!(*value, 42);
        } else {
            panic!("Expected LoadImmediate instruction");
        }
    }

    #[test]
    fn test_codegen_binary_operation() {
        let input = "let y = 1 + 2;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&parser).unwrap();

        let mut code_generator = CodeGenerator::new();
        let instructions = code_generator.generate(&ir_generator).expect("Code generation failed");

        assert_eq!(instructions.len(), 3);
        if let MachineInstruction::LoadImmediate { dest, value } = &instructions[0] {
            assert_eq!(dest, "t0");
            assert_eq!(*value, 1);
        } else {
            panic!("Expected LoadImmediate instruction");
        }
        if let MachineInstruction::LoadImmediate { dest, value } = &instructions[1] {
            assert_eq!(dest, "t1");
            assert_eq!(*value, 2);
        } else {
            panic!("Expected LoadImmediate instruction");
        }
        if let MachineInstruction::Add { dest, src1, src2 } = &instructions[2] {
            assert_eq!(dest, "t2");
            assert_eq!(src1, "t0");
            assert_eq!(src2, "t1");
        } else {
            panic!("Expected Add instruction");
        }
    }

    #[test]
    fn test_codegen_print_statement() {
        let input = "print x;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&parser).unwrap();

        let mut code_generator = CodeGenerator::new();
        let instructions = code_generator.generate(&ir_generator).expect("Code generation failed");

        assert_eq!(instructions.len(), 1);
        if let MachineInstruction::Print { var } = &instructions[0] {
            assert_eq!(var, "x");
        } else {
            panic!("Expected Print instruction");
        }
    }

    #[test]
    fn test_codegen_complex_expression() {
        let input = "let z = (1 + 2) * 3;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&parser).unwrap();

        let mut code_generator = CodeGenerator::new();
        let instructions = code_generator.generate(&ir_generator).expect("Code generation failed");

        assert_eq!(instructions.len(), 5);
        if let MachineInstruction::LoadImmediate { dest, value } = &instructions[0] {
            assert_eq!(dest, "t0");
            assert_eq!(*value, 1);
        } else {
            panic!("Expected LoadImmediate instruction");
        }
        if let MachineInstruction::LoadImmediate { dest, value } = &instructions[1] {
            assert_eq!(dest, "t1");
            assert_eq!(*value, 2);
        } else {
            panic!("Expected LoadImmediate instruction");
        }
        if let MachineInstruction::Add { dest, src1, src2 } = &instructions[2] {
            assert_eq!(dest, "t2");
            assert_eq!(src1, "t0");
            assert_eq!(src2, "t1");
        } else {
            panic!("Expected Add instruction");
        }
        if let MachineInstruction::LoadImmediate { dest, value } = &instructions[3] {
            assert_eq!(dest, "t3");
            assert_eq!(*value, 3);
        } else {
            panic!("Expected LoadImmediate instruction");
        }
        if let MachineInstruction::Mul { dest, src1, src2 } = &instructions[4] {
            assert_eq!(dest, "t4");
            assert_eq!(src1, "t2");
            assert_eq!(src2, "t3");
        } else {
            panic!("Expected Mul instruction");
        }
    }
}
