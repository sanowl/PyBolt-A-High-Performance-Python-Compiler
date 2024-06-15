use crate::ir::ir::{IRGenerator, IR};

#[derive(Debug)]
pub enum MachineInstruction {
    LoadImmediate { dest: String, value: i32 },
    Add { dest: String, src1: String, src2: String },
    Sub { dest: String, src1: String, src2: String },
    Mul { dest: String, src1: String, src2: String },
    Div { dest: String, src1: String, src2: String },
    Print { var: String },
}

pub struct CodeGenerator {
    instructions: Vec<MachineInstruction>,
    temp_var_counter: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            instructions: Vec::new(),
            temp_var_counter: 0,
        }
    }

    pub fn generate(&mut self, ir_generator: &IRGenerator) -> Result<Vec<MachineInstruction>, String> {
        let ir = ir_generator.get_ir();
        for node in ir {
            self.visit_node(node)?;
        }
        println!("Code generation completed.");
        Ok(self.instructions.clone())
    }

    fn visit_node(&mut self, node: &IR) -> Result<(), String> {
        match node {
            IR::VariableDeclaration { name, value } => {
                self.instructions.push(MachineInstruction::LoadImmediate {
                    dest: name.clone(),
                    value: *value,
                });
            }
            IR::BinaryOperation { operator, left, right } => {
                let left_temp = self.generate_temp_var();
                let right_temp = self.generate_temp_var();
                self.visit_node(left)?;
                self.visit_node(right)?;
                let instruction = match operator.as_str() {
                    "+" => MachineInstruction::Add {
                        dest: left_temp.clone(),
                        src1: self.extract_var_name(left)?,
                        src2: self.extract_var_name(right)?,
                    },
                    "-" => MachineInstruction::Sub {
                        dest: left_temp.clone(),
                        src1: self.extract_var_name(left)?,
                        src2: self.extract_var_name(right)?,
                    },
                    "*" => MachineInstruction::Mul {
                        dest: left_temp.clone(),
                        src1: self.extract_var_name(left)?,
                        src2: self.extract_var_name(right)?,
                    },
                    "/" => MachineInstruction::Div {
                        dest: left_temp.clone(),
                        src1: self.extract_var_name(left)?,
                        src2: self.extract_var_name(right)?,
                    },
                    _ => return Err(format!("Unknown operator: {}", operator)),
                };
                self.instructions.push(instruction);
            }
            IR::Immediate { value, dest } => {
                self.instructions.push(MachineInstruction::LoadImmediate {
                    dest: dest.clone(),
                    value: *value,
                });
            }
            IR::Print { var } => {
                self.instructions.push(MachineInstruction::Print { var: var.clone() });
            }
        }
        Ok(())
    }

    fn extract_var_name(&self, ir: &IR) -> Result<String, String> {
        match ir {
            IR::Immediate { dest, .. } => Ok(dest.clone()),
            _ => Err("Expected variable name".to_string()),
        }
    }

    fn generate_temp_var(&mut self) -> String {
        let temp_var = format!("t{}", self.temp_var_counter);
        self.temp_var_counter += 1;
        temp_var
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::ir::{IRGenerator, IR};
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    #[test]
    fn test_code_generation() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

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
    fn test_code_generation_with_binary_operation() {
        let input = "let y = 1 + 2;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

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
        if let M
