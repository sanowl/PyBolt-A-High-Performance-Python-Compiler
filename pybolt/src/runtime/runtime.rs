use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadImmediate { var: String, value: i32 },
    Add { dest: String, src1: String, src2: String },
    Sub { dest: String, src1: String, src2: String },
    Mul { dest: String, src1: String, src2: String },
    Div { dest: String, src1: String, src2: String },
    Print { var: String },
}

pub struct Runtime {
    instructions: Vec<Instruction>,
    symbol_table: HashMap<String, i32>,
}

impl Runtime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Runtime {
            instructions,
            symbol_table: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        for instruction in &self.instructions {
            self.execute(instruction)?;
        }
        Ok(())
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<(), String> {
        match instruction {
            Instruction::LoadImmediate { var, value } => {
                self.symbol_table.insert(var.clone(), *value);
            }
            Instruction::Add { dest, src1, src2 }
            | Instruction::Sub { dest, src1, src2 }
            | Instruction::Mul { dest, src1, src2 }
            | Instruction::Div { dest, src1, src2 } => {
                // Gather required values first
                let val1 = self.symbol_table.get(src1).cloned();
                let val2 = self.symbol_table.get(src2).cloned();
                
                // Ensure the values are available before proceeding
                let val1 = val1.ok_or(format!("Undefined variable '{}'", src1))?;
                let val2 = val2.ok_or(format!("Undefined variable '{}'", src2))?;
                
                // Perform the operation
                let result = match instruction {
                    Instruction::Add { .. } => val1 + val2,
                    Instruction::Sub { .. } => val1 - val2,
                    Instruction::Mul { .. } => val1 * val2,
                    Instruction::Div { .. } => val1 / val2,
                    _ => unreachable!(),
                };
                
                // Update symbol_table with the result
                self.symbol_table.insert(dest.clone(), result);
            }
            Instruction::Print { var } => {
                // Gather the value to print
                let value = self.symbol_table.get(var).cloned();
                
                // Ensure the value is available before proceeding
                let value = value.ok_or(format!("Undefined variable '{}'", var))?;
                
                // Print the value
                println!("{} = {}", var, value);
            }
        }
        Ok(())
    }

    pub fn get_var_value(&self, var: &str) -> Option<&i32> {
        self.symbol_table.get(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime() {
        let instructions = vec![
            Instruction::LoadImmediate { var: "x".to_string(), value: 42 },
            Instruction::LoadImmediate { var: "y".to_string(), value: 10 },
            Instruction::Add { dest: "z".to_string(), src1: "x".to_string(), src2: "y".to_string() },
            Instruction::Print { var: "z".to_string() },
        ];

        let mut runtime = Runtime::new(instructions);
        assert!(runtime.run().is_ok());
    }
}
