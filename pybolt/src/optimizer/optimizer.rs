use crate::ir::ir::{IR as IRNode, IRGenerator};
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

    pub fn optimize(&mut self, ir: Vec<IRNode>) -> Result<Vec<IRNode>, String> {
        let mut optimized_ir = ir;

        for optimization in &self.optimizations {
            optimized_ir = optimization.apply(optimized_ir)?;
        }

        println!("Optimization completed.");
        Ok(optimized_ir)
    }
}

pub trait Optimization {
    fn apply(&self, ir: Vec<IRNode>) -> Result<Vec<IRNode>, String>;
}

pub struct ConstantFolding;

impl Optimization for ConstantFolding {
    fn apply(&self, ir: Vec<IRNode>) -> Result<Vec<IRNode>, String> {
        let mut new_ir = Vec::new();

        for instruction in ir {
            match instruction {
                IRNode::BinaryOperation { left, operator, right } => {
                    if let (IRNode::Immediate { value: left_val, .. }, IRNode::Immediate { value: right_val, .. }) = (*left, *right) {
                        let result = match operator.as_str() {
                            "+" => left_val + right_val,
                            "-" => left_val - right_val,
                            "*" => left_val * right_val,
                            "/" => left_val / right_val,
                            _ => return Err(format!("Unknown operator: {}", operator)),
                        };
                        new_ir.push(IRNode::Immediate { value: result, dest: "temp".to_string() });
                    } else {
                        new_ir.push(IRNode::BinaryOperation { left, operator, right });
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
    fn apply(&self, ir: Vec<IRNode>) -> Result<Vec<IRNode>, String> {
        let mut used_vars = HashMap::new();
        let mut new_ir = Vec::new();

        for instruction in ir.iter().rev() {
            match instruction {
                IRNode::Immediate { dest, .. } => {
                    if used_vars.contains_key(dest) {
                        new_ir.push(instruction.clone());
                        used_vars.remove(dest);
                    }
                }
                IRNode::Print { var } => {
                    new_ir.push(instruction.clone());
                    used_vars.insert(var.clone(), true);
                }
                _ => new_ir.push(instruction.clone()),
            }
        }

        Ok(new_ir.into_iter().rev().collect())
    }
}
