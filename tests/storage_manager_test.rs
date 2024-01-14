use kvstore::tests::mocks::MockFile;

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::path::Path;
    use kvstore::storage_manager::Store;
    use kvstore::storage_manager::StoreMetaData;

    #[test]
    fn open_and_close_store() {
        let path = Path::new("test.db");
        let store = Store::open_store(&path).unwrap();

        // Verify store opened
        // assert!(store.metadata().is_ok());

        // close store
        drop(store);

        // verify closed 
        assert!(Store::open_store(&path).is_ok());
        // Cleanup
        let _ = std::fs::remove_file(path);
    } 

    #[test]
    fn test_metadata() {
        let path = Path::new("testmetadata.test");
        let store = Store::open_store(&path).unwrap();

        let metadata = store.metadata().unwrap();
        let capacity = metadata.get_capacity();
        let num_entries = metadata.get_entries();
        let version = metadata.get_version();
        let signature = metadata.get_signature();

        assert_eq!(capacity, file_size);
        assert_eq!(num_entries, 0);
        assert_ne!(signature, 0);
        assert_eq!(version, 1);

        // Error case 
        let mock_store = Store {file: File};
        assert!(mock_store.metadata().is_err());
    } 
    #[test] 
    fn test_calculate_signature() {
      let mut metadata = StoreMetaData::new();
      let sig1 = metadata.calculate_signature();


      let mut capacity = metadata.get_capacity();
      capacity += 1; 
      let sig2 = metadata.calculate_signature();

      assert_ne!(sig1, sig2);
    } 

    #[test]
    fn test_drop() {
      let path = Path::new("testmetadata.test");
      let store = Store::open_store(path).unwrap();
      
      // Mock flush to test sync 
      let mock_file = store.get_file();
      store.file = MockFile::new();

      drop(store);

      assert!(mock_file.flushed);
      assert!(mock_file.synced); 
    }
}
