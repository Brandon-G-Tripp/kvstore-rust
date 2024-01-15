use core::fmt;
use std::{fs::{OpenOptions, File}, path::Path, io::{Write, Seek, self}, error::Error, collections::hash_map::DefaultHasher };
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

pub trait SyncFile: Write + Seek {
    fn sync_all(&self);

    fn sync_data(&self) -> Result<(), io::Error>;
} 

pub struct Store {
    file: File
}

#[derive(Debug)]
enum StoreError {
    Io(std::io::Error),
    Serialization(String),
    Corruption {msg: String},
} 

impl From<std::io::Error> for StoreError {
    fn from(err: std::io::Error) -> StoreError {
        StoreError::Io(err)
    } 
} 

impl From<StoreError> for std::io::Error {
    fn from(error: StoreError) ->  std::io::Error {
        match error {
            StoreError::Io(io_error) => io_error,
            _ => std::io::Error::new(std::io::ErrorKind::Other, "Store error")
        } 
    }
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
            .open(path)
            .map_err(StoreError::from)?;

        Ok(Store{file})
    } 

    pub fn metadata<E: Error>(&self) -> Result<StoreMetaData, MetaDataError> {
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

#[derive(Debug)]
pub struct MetaDataError {
    details: String, 
} 

impl fmt::Display for MetaDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Metadata error: {}", self.details)
    }
} 

impl Error for MetaDataError {
    fn description(&self) -> &str {
        &self.details
    }
} 

impl From<std::io::Error> for MetaDataError {
    fn from(error: std::io::Error) -> Self { 
        MetaDataError {
            details: error.to_string()
        } 
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test] 
    fn test_open_and_create_store() {
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");

        let _store = Store::open_store(&store_path).unwrap();

        // Fiel should now exist 
        assert!(store_path.exists());

        // Geting file should return a valid handle
    } 

    #[test]
    fn test_metadata() {
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");

        let store = Store::open_store(&store_path).unwrap();

        let metadata = store.metadata::<MetaDataError>().unwrap();

        // Assert on metadata fields 
        assert_eq!(metadata.capacity, 0);
        assert_eq!(metadata.num_entries, 0);
        assert_eq!(metadata.version, 1);

        let calculated_signature = metadata.calculate_signature();

        // Signature shouuld match 
        assert_eq!(metadata.signature, calculated_signature);
    } 

} 
