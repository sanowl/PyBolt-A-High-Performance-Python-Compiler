use crate::parser::parser::Parser;

pub struct IRGenerator;

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator
    }

    pub fn generate(&mut self, parser: &Parser) -> Result<(), String> {
        println!("IR generation completed.");
        Ok(())
    }
}
