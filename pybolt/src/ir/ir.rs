use crate::parser::parser::{Parser, ASTNode, ASTNodeType};

#[derive(Debug, Clone)]
pub enum IR {
    VariableDeclaration { name: String, value: i32 },
    BinaryOperation { operator: String, left: Box<IR>, right: Box<IR> },
    Immediate { value: i32, dest: String },
    Print { var: String },
}

pub struct IRGenerator {
    ir: Vec<IR>,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator { ir: Vec::new() }
    }

    pub fn generate(&mut self, parser: &Parser) -> Result<Vec<IR>, String> {
        let ast = parser.get_ast();
        self.visit_node(&ast)?;
        println!("IR generation completed.");
        Ok(self.ir.clone())
    }

    fn visit_node(&mut self, node: &ASTNode) -> Result<(), String> {
        match node {
            ASTNode::Program(nodes) => {
                for child in nodes {
                    self.visit_node(child)?;
                }
            }
            ASTNode::VariableDeclaration { name, value } => {
                if let ASTNode::Number(n) = **value {
                    self.ir.push(IR::VariableDeclaration { name: name.clone(), value: n });
                } else {
                    return Err("Expected a number in variable declaration".to_string());
                }
            }
            ASTNode::BinaryOperation { operator, left, right } => {
                let left_ir = self.generate_ir_from_node(left)?;
                let right_ir = self.generate_ir_from_node(right)?;
                self.ir.push(IR::BinaryOperation {
                    operator: operator.clone(),
                    left: Box::new(left_ir),
                    right: Box::new(right_ir),
                });
            }
            ASTNode::Number(n) => {
                self.ir.push(IR::Immediate { value: *n, dest: "temp".to_string() });
            }
            ASTNode::Expression(ASTNodeType::Identifier(name)) => {
                self.ir.push(IR::Print { var: name.clone() });
            }
            _ => return Err("Unsupported AST node type".to_string()),
        }
        Ok(())
    }

    fn generate_ir_from_node(&mut self, node: &ASTNode) -> Result<IR, String> {
        match node {
            ASTNode::Number(n) => Ok(IR::Immediate { value: *n, dest: "temp".to_string() }),
            ASTNode::BinaryOperation { operator, left, right } => {
                let left_ir = self.generate_ir_from_node(left)?;
                let right_ir = self.generate_ir_from_node(right)?;
                Ok(IR::BinaryOperation {
                    operator: operator.clone(),
                    left: Box::new(left_ir),
                    right: Box::new(right_ir),
                })
            }
            ASTNode::Expression(ASTNodeType::Identifier(name)) => Ok(IR::Print { var: name.clone() }),
            _ => Err("Unsupported AST node type".to_string()),
        }
    }

    // Add the get_ir method
    pub fn get_ir(&self) -> &Vec<IR> {
        &self.ir
    }
}
