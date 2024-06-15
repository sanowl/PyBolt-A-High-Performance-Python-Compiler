pub mod runtime;

#[cfg(test)]
mod tests {
    use super::runtime::{Runtime, Instruction};

    #[test]
    fn test_runtime_execution() {
        let instructions = vec![
            Instruction::LoadImmediate { var: "x".to_string(), value: 42 },
            Instruction::Print { var: "x".to_string() },
        ];

        let mut runtime = Runtime::new(instructions);
        assert!(runtime.run().is_ok());
    }

    #[test]
    fn test_arithmetic_operations() {
        let instructions = vec![
            Instruction::LoadImmediate { var: "a".to_string(), value: 10 },
            Instruction::LoadImmediate { var: "b".to_string(), value: 5 },
            Instruction::Add { dest: "sum".to_string(), src1: "a".to_string(), src2: "b".to_string() },
            Instruction::Sub { dest: "diff".to_string(), src1: "a".to_string(), src2: "b".to_string() },
            Instruction::Mul { dest: "prod".to_string(), src1: "a".to_string(), src2: "b".to_string() },
            Instruction::Div { dest: "quot".to_string(), src1: "a".to_string(), src2: "b".to_string() },
            Instruction::Print { var: "sum".to_string() },
            Instruction::Print { var: "diff".to_string() },
            Instruction::Print { var: "prod".to_string() },
            Instruction::Print { var: "quot".to_string() },
        ];

        let mut runtime = Runtime::new(instructions);
        assert!(runtime.run().is_ok());
    }

    #[test]
    fn test_error_handling() {
        let instructions = vec![
            Instruction::LoadImmediate { var: "x".to_string(), value: 42 },
            Instruction::Add { dest: "y".to_string(), src1: "x".to_string(), src2: "z".to_string() }, // 'z' is undefined
        ];

        let mut runtime = Runtime::new(instructions);
        assert!(runtime.run().is_err());
    }

    #[test]
    fn test_variable_usage() {
        let instructions = vec![
            Instruction::LoadImmediate { var: "x".to_string(), value: 5 },
            Instruction::LoadImmediate { var: "y".to_string(), value: 10 },
            Instruction::Add { dest: "z".to_string(), src1: "x".to_string(), src2: "y".to_string() },
            Instruction::Print { var: "z".to_string() },
        ];

        let mut runtime = Runtime::new(instructions);
        runtime.run().unwrap();

        assert_eq!(runtime.get_var_value("x"), Some(&5));
        assert_eq!(runtime.get_var_value("y"), Some(&10));
        assert_eq!(runtime.get_var_value("z"), Some(&15));
    }
}
