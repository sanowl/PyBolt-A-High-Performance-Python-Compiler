use crate::ir::ir::IRGenerator;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator
    }

    pub fn generate(&mut self, ir_generator: &IRGenerator) -> Result<(), String> {
        println!("Code generation completed.");
        Ok(())
    }
}
