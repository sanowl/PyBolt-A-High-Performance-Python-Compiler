#[cfg(test)]
mod runtime_tests {
    use crate::runtime::runtime::Runtime;

    #[test]
    fn test_runtime() {
        let mut runtime = Runtime::new();
        assert!(runtime.run().is_ok());
    }
}
