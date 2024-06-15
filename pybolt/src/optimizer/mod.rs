pub mod optimizer;

#[cfg(test)]
mod tests {
    use super::optimizer::Optimizer;
    use crate::ir::ir::{IRGenerator, IR};
    use crate::parser::parser::Parser;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_constant_folding() {
        let input = "let x = 2 + 3;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&ast).unwrap();

        let mut optimizer = Optimizer::new();
        let optimized_ir = optimizer.optimize(ir).expect("Optimization failed");

        // Check if the constant folding optimization was applied
        assert_eq!(optimized_ir.len(), 1);
        if let IR::Immediate { value, dest } = &optimized_ir[0] {
            assert_eq!(*value, 5);
            assert_eq!(dest, "x");
        } else {
            panic!("Expected an immediate instruction with folded constant");
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let input = "let x = 42; let y = x + 5; let z = 100;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&ast).unwrap();

        let mut optimizer = Optimizer::new();
        let optimized_ir = optimizer.optimize(ir).expect("Optimization failed");

        // Check if the dead code elimination optimization was applied
        let used_vars = optimized_ir.iter().map(|inst| match inst {
            IR::Variable { name } => name.clone(),
            _ => panic!("Expected variable instructions"),
        }).collect::<Vec<_>>();
        assert!(used_vars.contains(&"x".to_string()));
        assert!(used_vars.contains(&"y".to_string()));
        assert!(!used_vars.contains(&"z".to_string()));
    }

    #[test]
    fn test_combined_optimization() {
        let input = "let a = 2 + 3; let b = a * 10; let c = 100;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

        let mut ir_generator = IRGenerator::new();
        let ir = ir_generator.generate(&ast).unwrap();

        let mut optimizer = Optimizer::new();
        let optimized_ir = optimizer.optimize(ir).expect("Optimization failed");

        // Check if both optimizations (constant folding and dead code elimination) were applied
        let used_vars = optimized_ir.iter().map(|inst| match inst {
            IR::Variable { name } => name.clone(),
            _ => panic!("Expected variable instructions"),
        }).collect::<Vec<_>>();
        assert!(used_vars.contains(&"a".to_string()));
        assert!(used_vars.contains(&"b".to_string()));
        assert!(!used_vars.contains(&"c".to_string()));
    }
}
