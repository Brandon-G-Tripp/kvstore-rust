#[cfg(test)]
mod tests {
    use kvstore::keys::Key;
    use kvstore::keys::Value;

    #[test]
    fn test_key_creation() {
        let k1 = Key::new();
        let k1_id = k1.get_id();
        assert_eq!(k1.get_id(),  k1_id);
    } 

    #[test]
    fn test_key_eq() {
        let k1 = Key::new();
        let k2 = Key::new();
        assert!(k1 != k2);
    } 

    #[test]
    fn test_value_creation() {
        let mut v = Value::default();
        v.set(5);
        assert_eq!(v.get(), &5);
    }

    #[test]
    fn test_value_debug() {
        let v = Value::new("test");
        assert_eq!(format!("{:?}", v), "\"test\"");

    } 
}
