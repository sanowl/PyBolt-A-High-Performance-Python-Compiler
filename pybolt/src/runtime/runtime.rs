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
                let val1 = *self.symbol_table.get(src1).ok_or(format!("Undefined variable '{}'", src1))?;
                let val2 = *self.symbol_table.get(src2).ok_or(format!("Undefined variable '{}'", src2))?;
                let result = match instruction {
                    Instruction::Add { .. } => val1 + val2,
                    Instruction::Sub { .. } => val1 - val2,
                    Instruction::Mul { .. } => val1 * val2,
                    Instruction::Div { .. } => val1 / val2,
                    _ => unreachable!(),
                };
                self.symbol_table.insert(dest.clone(), result);
            }
            Instruction::Print { var } => {
                if let Some(value) = self.symbol_table.get(var) {
                    println!("{} = {}", var, value);
                } else {
                    return Err(format!("Undefined variable '{}'", var));
                }
            }
        }
        Ok(())
    }

    pub fn get_var_value(&self, var: &str) -> Option<&i32> {
        self.symbol_table.get(var)
    }
}
