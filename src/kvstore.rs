trait KVStore {
    fn get(&self, key: String) -> Option<Value>;

    fn put(&mut self, key: String, value: Value);

    fn delete(&mut self, key: String);
} 

#[cfg(test)]
mod tests {
    use super::*;

    impl KVStore for MockKVStore {
    }

    #[test] 
    fn get_missing_key() {
        let store = MockKVStore;

        assert_eq!(None, store.get("key".to_string()));
    } 
} 
