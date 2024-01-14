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
}
