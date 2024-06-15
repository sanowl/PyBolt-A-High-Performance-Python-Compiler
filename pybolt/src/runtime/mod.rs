pub mod runtime;

#[cfg(test)]
mod tests {
    use super::runtime::Runtime;

    #[test]
    fn test_runtime() {
        let mut runtime = Runtime::new();
        assert!(runtime.run().is_ok());
    }
}
