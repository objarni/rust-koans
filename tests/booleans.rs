#[cfg(test)]
mod tests {

    #[test]
    #[allow(unused_variables)]
    fn test_declare_constant_boolean_var() {
        let b: bool;
    }

    #[test]
    fn test_declare_prefix_underscore_to_ignore_dead_var() {
        let _b: bool;
    }

    #[test]
    fn test_true() {
        assert!(true)
    }

    #[test]
    fn test_false() {
        assert!(!false)
    }

    #[test]
    fn parse_true() {
        assert!("true".parse().unwrap())
    }
}