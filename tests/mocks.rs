pub struct MockFile {
    pub flushed: bool,
    pub synced: bool 
}

impl MockFile {

    pub fn new() -> MockFile {
        MockFile {
            flushed: false,
            synced: false  
        }
    }

}
