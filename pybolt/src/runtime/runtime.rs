pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Runtime
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("Runtime executed.");
        Ok(())
    }
}
