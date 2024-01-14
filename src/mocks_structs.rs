use std::io::{Write, Seek, Cursor, Result, Read};
use crate::storage_manager::{SyncFile};

pub struct MockFile {
    pub flushed: bool,
    pub synced: bool,  
    read_called: bool,
    last_read_buf: Vec<u8>,
    pub file: Cursor<Vec<u8>>
}

impl MockFile {

    pub fn new() -> MockFile {
        let file = Cursor::new(Vec::new()); // in memory buffer
        MockFile {
            file,
            last_read_buf: Vec::new(),
            read_called: false,
            flushed: false,
            synced: false
        }
    }

    pub fn set_synced(&mut self, synced: bool) {
        self.synced = synced;
    } 

}

impl SyncFile for MockFile {
    fn sync_all(&self) {}

    fn sync_data(&self) -> Result<()> {
        Ok(())
    } 
}

impl Write for MockFile {
    fn flush(&mut self) -> std::io::Result<()> {
        self.flushed = true;
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
} 

impl Seek for MockFile {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.synced = true;
        Ok(0)
    }
} 

impl Read for MockFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_called = true;
        self.last_read_buf = buf.to_vec();
        Ok(buf.len())
    } 

    fn read_exact(&mut self, _buf: &mut [u8]) -> Result<()> {
        Ok(())
    }
} 
