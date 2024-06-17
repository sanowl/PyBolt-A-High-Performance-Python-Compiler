use crate::ir::ir::{IRGenerator, IR};

#[derive(Debug, Clone)]
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
                self.instructions.push(MachineInstruction::LoadImmediate {
                    dest: left_temp.clone(),
                    value: self.extract_immediate_value(left)?,
                });
                self.instructions.push(MachineInstruction::LoadImmediate {
                    dest: right_temp.clone(),
                    value: self.extract_immediate_value(right)?,
                });
                let result_var = self.generate_temp_var();
                let instruction = match operator.as_str() {
                    "+" => MachineInstruction::Add {
                        dest: result_var.clone(),
                        src1: left_temp,
                        src2: right_temp,
                    },
                    "-" => MachineInstruction::Sub {
                        dest: result_var.clone(),
                        src1: left_temp,
                        src2: right_temp,
                    },
                    "*" => MachineInstruction::Mul {
                        dest: result_var.clone(),
                        src1: left_temp,
                        src2: right_temp,
                    },
                    "/" => MachineInstruction::Div {
                        dest: result_var.clone(),
                        src1: left_temp,
                        src2: right_temp,
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

    fn extract_immediate_value(&self, ir: &IR) -> Result<i32, String> {
        match ir {
            IR::Immediate { value, .. } => Ok(*value),
            _ => Err("Expected immediate value".to_string()),
        }
    }

    fn generate_temp_var(&mut self) -> String {
        let temp_var = format!("t{}", self.temp_var_counter);
        self.temp_var_counter += 1;
        temp_var
    }
}
