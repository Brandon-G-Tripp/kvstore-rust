use std::collections::BTreeMap;

use crate::values::Value;

trait KVStore<T> {
    fn get(&self, key: String) -> Option<&Value<T>>;

    fn put(&mut self, key: String, value: Value<T>);

    fn delete(&mut self, key: String);
} 

struct MockKVStore<T> {
    data: BTreeMap<String, Value<T>>
} 

impl<T> KVStore<T> for MockKVStore<T> {
    fn get(&self, key: String) -> Option<&Value<T>> {
        self.data.get(&key)
    }

    fn put(&mut self, key: String, value: Value<T>) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }
} 

impl<T> MockKVStore<T> {

    fn new() -> Self {
        Self {
            data: BTreeMap::new()
        }
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn get_missing_key() {
        let store:  MockKVStore<i32> = MockKVStore::new();

        assert_eq!(None, store.get("key".to_string()));
    } 

    #[test]
    fn put_inserts_key_value() {
        let mut store = MockKVStore::<i32>::new();

        store.put("key".to_string(), Value::new(1));

        let optional_value = store.get("key".to_string());
        let value = optional_value.unwrap();

        assert_eq!(Value::new(1), *value);
    }

    #[test]  
    fn delete_removes_key() {
      let mut store = MockKVStore::new();
      
      store.put("key".to_string(), Value::new(1));
      
      store.delete("key".to_string());
      
      assert_eq!(None, store.get("key".to_string()))  
    }
} 
