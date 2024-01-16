use kvstore::{keys::Key, values::Value};
use kvstore::page::PageFormat;
use kvstore::storage_manager::Store;
use std::env;
use std::path::Path;



#[test]
fn test_write_and_read_page() {
    let tmp_dir = env::temp_dir();
    let store_path = tmp_dir.join("test_store");
    let path = Path::new(&store_path);

    let mut store = Store::new(&path).unwrap();

    let id = store.allocate_page();
    let mut page = PageFormat::new();

    store.write_page(&mut page, &id).unwrap();

    let read = store.read_page(id).unwrap();
    assert_eq!(page, read);
} 

// #[test]
// fn test_insert_and_get() {
//     let tmp_dir = env::temp_dir();
//     let store_path = tmp_dir.join("test_store");
//     let path = Path::new(&store_path);

//     let mut store = Store::new(&path).unwrap();

//     let key = Key::new();
//     let value = Value::new("test");

//     store.insert(key, value);

//     let read = store.get(&key).unwrap();
//     assert_eq!(read.get(), "test");
// } 

#[test]
fn test_multiple_pages() {
    let tmp_dir = env::temp_dir();
    let store_path = tmp_dir.join("test_store");
    let path = Path::new(&store_path);
    
    let mut store = Store::new(&path).unwrap();

    for i in 0..5 {
        let id = store.allocate_page();
        let mut page = PageFormat::new();
    
        store.write_page(&mut page, &id).unwrap();
    }
    
    assert_eq!(store.len(), 5); 
} 
