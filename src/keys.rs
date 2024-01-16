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

