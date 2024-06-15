use crate::ir::ir::{IR, IRGenerator};
use std::collections::HashMap;

pub struct Optimizer {
    optimizations: Vec<Box<dyn Optimization>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            optimizations: vec![Box::new(ConstantFolding), Box::new(DeadCodeElimination)],
        }
    }

    pub fn optimize(&mut self, ir: Vec<IR>) -> Result<Vec<IR>, String> {
        let mut optimized_ir = ir;

        for optimization in &self.optimizations {
            optimized_ir = optimization.apply(optimized_ir)?;
        }

        println!("Optimization completed.");
        Ok(optimized_ir)
    }
}

pub trait Optimization {
    fn apply(&self, ir: Vec<IR>) -> Result<Vec<IR>, String>;
}

pub struct ConstantFolding;

impl Optimization for ConstantFolding {
    fn apply(&self, ir: Vec<IR>) -> Result<Vec<IR>, String> {
        let mut new_ir = Vec::new();

        for instruction in ir {
            match instruction {
                IR::BinaryOperation { left, operator, right, dest } => {
                    if let (IR::Immediate { value: left_val }, IR::Immediate { value: right_val }) = (*left, *right) {
                        let result = match operator.as_str() {
                            "+" => left_val + right_val,
                            "-" => left_val - right_val,
                            "*" => left_val * right_val,
                            "/" => left_val / right_val,
                            _ => return Err(format!("Unknown operator: {}", operator)),
                        };
                        new_ir.push(IR::Immediate { value: result, dest });
                    } else {
                        new_ir.push(IR::BinaryOperation { left, operator, right, dest });
                    }
                }
                _ => new_ir.push(instruction),
            }
        }

        Ok(new_ir)
    }
}

pub struct DeadCodeElimination;

impl Optimization for DeadCodeElimination {
    fn apply(&self, ir: Vec<IR>) -> Result<Vec<IR>, String> {
        let mut used_vars = HashMap::new();
        let mut new_ir = Vec::new();

        for instruction in ir.iter().rev() {
            match instruction {
                IR::BinaryOperation { dest, .. } | IR::Immediate { dest, .. } => {
                    if used_vars.contains_key(dest) {
                        new_ir.push(instruction.clone());
                        used_vars.remove(dest);
                    }
                }
                IR::Print { var } => {
                    new_ir.push(instruction.clone());
                    used_vars.insert(var.clone(), true);
                }
                _ => new_ir.push(instruction.clone()),
            }
        }

        Ok(new_ir.into_iter().rev().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::ir::{IR, IRGenerator};

    #[test]
    fn test_constant_folding() {
        let ir = vec![
            IR::BinaryOperation {
                left: Box::new(IR::Immediate { value: 2 }),
                operator: "+".to_string(),
                right: Box::new(IR::Immediate { value: 3 }),
                dest: "x".to_string(),
            },
        ];

        let mut optimizer = Optimizer::new();
        let optimized_ir = optimizer.optimize(ir).expect("Optimization failed");

        if let IR::Immediate { value, dest } = &optimized_ir[0] {
            assert_eq!(*value, 5);
            assert_eq!(dest, "x");
        } else {
            panic!("Expected immediate instruction");
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let ir = vec![
            IR::Immediate { value: 42, dest: "x".to_string() },
            IR::Immediate { value: 43, dest: "y".to_string() },
            IR::Print { var: "x".to_string() },
        ];

        let mut optimizer = Optimizer::new();
        let optimized_ir = optimizer.optimize(ir).expect("Optimization failed");

        assert_eq!(optimized_ir.len(), 2);
        if let IR::Immediate { value, dest } = &optimized_ir[0] {
            assert_eq!(*value, 42);
            assert_eq!(dest, "x");
        } else {
            panic!("Expected immediate instruction");
        }
        if let IR::Print { var } = &optimized_ir[1] {
            assert_eq!(var, "x");
        } else {
            panic!("Expected print instruction");
        }
    }
}
