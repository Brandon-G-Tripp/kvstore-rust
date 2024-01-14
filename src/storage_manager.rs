use std::{fs::{OpenOptions, File}, path::Path, io::Write, error::Error, collections::hash_map::DefaultHasher };
use std::hash::Hasher;

struct PageFormat {
    header: [u8; 16],
    slot_cap: u32,
    num_slots: u32,
    slots: [u8; 4096],
    footer: [u8; 16]
} 

pub struct StoreMetaData {
    capacity: u64,
    num_entries: u32,
    signature: u64,
    version: u32
} 

pub struct Store {
    file: File
}

impl Drop for Store {
    fn drop(&mut self) {
        let _ = self.file.flush();
        let _ = self.file.sync_all();
        self.file.sync_data().unwrap();
    }
}

impl Store {

    pub fn open_store(path: &Path) -> Result<Store, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Store{file})
    } 

    pub fn metadata<E: Error>(&self) -> Result<StoreMetaData, std::io::Error> {
        let mut metadata = StoreMetaData {
            capacity: self.file.metadata()?.len(),
            num_entries: 0, 
            signature: 0,
            version: 1
        };

        metadata.signature = metadata.calculate_signature();

        Ok(metadata)
    } 

    pub fn get_file(&self) -> &File {
        &self.file
    } 
}

impl StoreMetaData {

    pub fn new() -> StoreMetaData {
        StoreMetaData {
          capacity: 0,
          num_entries: 0,
          signature: 0,
          version: 0  
        }
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity
    } 

    pub fn get_entries(&self) -> u32 {
        self.num_entries
    } 

    pub fn get_signature(&self) -> u64 {
        self.signature
    } 
    
    pub fn get_version(&self) -> u32 {
        self.version
    } 

    pub fn calculate_signature(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        // Hash pieces of metadata using Hasher
        hasher.write(&self.capacity.to_ne_bytes());
        hasher.write(&self.num_entries.to_ne_bytes());
        hasher.write(&self.version.to_ne_bytes());

        // Finish and return has as u64
        hasher.finish()
    }
}
