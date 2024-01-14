use std::hash::Hash;
use std::fmt::{Debug, Formatter};
use std::fmt::Result;
use uuid::Uuid;

pub struct Key {
    id: Uuid
} 

impl Key {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4()
        } 
    }

    pub fn get_id(&self) -> &Uuid {
        return &self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    } 
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    } 
} 

impl Eq for Key {}

impl Hash for Key {
    fn hash<H:  std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

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
