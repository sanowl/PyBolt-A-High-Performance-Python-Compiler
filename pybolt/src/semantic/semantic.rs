use crate::parser::parser::Parser;

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer
    }

    pub fn analyze(&mut self, parser: &Parser) -> Result<(), String> {
        println!("Semantic analysis completed.");
        Ok(())
    }
}
