use crate::ir::ir::{IR, IRGenerator};
use std::collections::HashMap;

pub enum IR {
    VariableDeclaration { name: String, value: i32 },
    BinaryOperation { left: Box<IR>, operator: String, right: Box<IR>, dest: String },
    Immediate { value: i32, dest: String },
    Print { var: String },
}

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
                    if let (IR::Immediate { value: left_val, dest: _ }, IR::Immediate { value: right_val, dest: _ }) = (*left, *right) {
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
