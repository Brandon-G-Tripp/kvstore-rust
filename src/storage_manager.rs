use core::fmt;
use std::{fs::{OpenOptions, File}, path::Path, io::{Write, Seek, self, SeekFrom, ErrorKind, Read}, error::Error, collections::{hash_map::DefaultHasher, BTreeMap}, sync::atomic::{AtomicUsize, Ordering} };
use std::hash::Hasher;

use crate::page::PageFormat;

pub trait SyncFile: Write + Seek {
    fn sync_all(&self);

    fn sync_data(&self) -> Result<(), io::Error>;
} 

pub struct Store {
    file: File,
    page_map: PageMap,
    num_pages: u32, 
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

    pub fn new(path: &Path) -> Result<Store, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .map_err(StoreError::from)?;

        Ok(
            Store {
                file,
                page_map: PageMap::new(),
                num_pages: 0,
            }
        )
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

    fn map_page(&mut self, page_id: &PageId, location: u64) {
        let id = page_id.clone();
        self.page_map.map_page(id, location);
    } 

    fn get_page_location(&self, page_id: &PageId) -> Option<u64> {
        self.page_map.get_location(page_id)
    }

    pub fn len(&self) -> u32 {
        self.num_pages
    } 

    pub fn allocate_page(&mut self) -> PageId {
        let id = PageId::new();
        let cloned_id = id.clone();
        let location = self.allocate_page_on_disk();
        self.map_page(&cloned_id, location);

        if let Some(_loc) = self.get_page_location(&id) {
            self.num_pages += 1;
            id
        } else {
            panic!("Failed to allocate page");
        } 
    } 

    fn allocate_page_on_disk(&mut self) -> u64 {
        let page = PageFormat::new();

        let page_location = self.file.metadata().unwrap().len();

        page.write_to_disk(&mut self.file).unwrap();

        page_location
    }

    fn serialize_page(&self, page: &mut PageFormat) -> Vec<u8> {
        page.serialize()
    }

    fn deserialize_page(&self, bytes: Vec<u8>) -> PageFormat {
        PageFormat::deserialize(bytes)
    }


    pub fn read_page(&mut self, id: PageId) -> Result<PageFormat, io::Error> {
        // Get allocated location
        let location = self.get_page_location(&id)
            .ok_or(io::Error::new(ErrorKind::Other, "Page not allocated"))?;

        // Seek to lcoation 
        self.file.seek(SeekFrom::Start(location))?;

        // Read page bytes from file 
        let mut bytes = vec![0; 4096];
        self.file.read_exact(&mut bytes)?;

        // Deserialize bytes to page 
        let page = PageFormat::deserialize(bytes);

        Ok(page)
    }

    pub fn write_page(&mut self, page: &PageFormat, id: &PageId) -> Result<(), io::Error> {
        // Get the allocated location for this page id 
        let location = self.get_page_location(id)
            .ok_or(io::Error::new(ErrorKind::Other, "Page not allocated"))?;

        // Seek to the location in the file 
        self.file.seek(SeekFrom::Start(location))?;

        // Serialize the page to get the bytes 
        let bytes = page.serialize();


        // Write bytes to file 
        self.file.write_all(&bytes)?;

        Ok(())
    } 

    pub fn put(&mut self, key: String, value: Value) {
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


pub struct StoreMetaData {
    capacity: u64,
    num_entries: u32,
    signature: u64,
    version: u32
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PageMap {
    mappings: BTreeMap<PageId, u64>
} 


#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct PageId {
    id: usize
}

impl PartialEq for PageId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    } 
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

impl PageId {

    fn new() -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed)}
    } 

    fn clone(&self) -> Self {
        Self { id: self.id }
    } 
} 

impl PageMap {
    fn new() -> Self {
        Self {
            mappings: BTreeMap::new()
        } 
    } 

    fn map_page(&mut self, page_id: PageId, location: u64) {
        self.mappings.insert(page_id, location);
    } 

    fn get_location(&self, page_id: &PageId) -> Option<u64> {
        self.mappings.get(page_id).copied()
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
        let path = Path::new(&store_path);

        let _store = Store::new(&path).unwrap();

        // Fiel should now exist 
        assert!(store_path.exists());
    } 

    #[test]
    fn test_write_and_read_page() {
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");
        let path = Path::new(&store_path);

        let mut store = Store::new(&path).unwrap();

        let id = store.allocate_page();

        let mut page = PageFormat::new();

        // Write Page 
        store.write_page(&mut page, &id).unwrap();

        // read page 
        let read = store.read_page(id).unwrap();

        assert_eq!(page, read);
    } 

    #[test]
    fn test_serialize_page() {
        // Create store 
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");
        let path = Path::new(&store_path);
        let store = Store::new(&path).unwrap();

        // Create page 
        let mut page = PageFormat::new();

        //Serialize Page 
        let bytes = store.serialize_page(&mut page);

        // Deserialize bytes 
        let deserialized = store.deserialize_page(bytes);

        // Pages should match 
        assert_eq!(page, deserialized)
    } 

    #[test]
    fn test_metadata() {
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");

        let store = Store::new(&store_path).unwrap();

        let metadata = store.metadata::<MetaDataError>().unwrap();

        // Assert on metadata fields 
        assert!(metadata.capacity >= 4096);
        assert_eq!(metadata.num_entries, 0);
        assert_eq!(metadata.version, 1);

        let calculated_signature = metadata.calculate_signature();

        // Signature shouuld match 
        assert_eq!(metadata.signature, calculated_signature);
    } 

    #[test]
    fn test_map_page_to_location() {
        
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");

        let mut store = Store::new(&store_path).unwrap();

        let page_id = PageId{id: 0};
        let location = 0;

        store.map_page(&page_id, location);

        assert_eq!(store.get_page_location(&page_id), Some(location));
    } 

    #[test]
    fn test_get_page_location() {
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");

        let store = Store::new(&store_path).unwrap();

        let page_id = PageId{id: 1};

        assert!(store.get_page_location(&page_id).is_none());
    } 

    #[test]
    fn test_allocate_page() {
        let tmp_dir = env::temp_dir();
        let store_path = tmp_dir.join("test_store");

        let mut store = Store::new(&store_path).unwrap();

        let page_id = store.allocate_page();

        assert!(store.get_page_location(&page_id).is_some());
    } 

    #[test]
    fn test_kv_put() {
        let mut store = Store::new();
    } 

} 
