use std::fmt::{Debug, Formatter};
use std::fmt::Result;

pub struct Value<T> {
    data: T
} 

impl<T> Value<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    } 

    pub fn get(&self) -> &T {
        &self.data
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    } 
} 

impl<T: Default> Default for Value<T> {
    fn default() -> Self {
        Self { data: Default::default() }
    } 
} 

impl<T: Debug> Debug for Value<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.data)
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_value_creation() {
        let mut v = Value::default();
        v.set(5);
        assert_eq!(v.get(), &5);
    }

    #[test]
    pub fn test_value_debug() {
        let v = Value::new("test");
        assert_eq!(format!("{:?}", v), "\"test\"");

    } 

    #[test]
    fn test_string_value() {
        let v = Value::new("foo");

        assert_eq!(v.get(), &"foo")
    }

    #[test] 
    fn test_vector_value() {
        let v = Value::new(vec![1,2,3]);

        assert_eq!(v.get(), &vec![1,2,3]);
    } 

    // Default values 
    #[test]
    fn test_default_integer() {
        let v: Value<i32> = Default::default();
        
        let value = *v.get();

        assert_eq!(value, 0);

        let is_default = value == 0;
        assert!(is_default);
    }

    #[test]
    fn test_default_boolean() {
        let v: Value<bool> = Default::default();

        assert!(!v.get());
    }

    // Mutable updates 
    #[test]
    fn test_mutable_updates() {
        let mut v = Value::new(1);

        v.set(2);

        assert_eq!(*v.get(), 2);
    } 
}
