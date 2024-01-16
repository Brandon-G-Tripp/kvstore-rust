const TOTAL_PAGE_SIZE: usize = 4096;

const NUM_SLOTS_SIZE: usize = 4;
const SLOT_CAP_SIZE: usize = 4;
const FOOTER_HEADER_SIZE: usize = 16;

const METADATA_SIZE: usize = FOOTER_HEADER_SIZE * 2 + NUM_SLOTS_SIZE + SLOT_CAP_SIZE;
const SLOTS_SIZE: usize = TOTAL_PAGE_SIZE - METADATA_SIZE;

use std::{fmt::Debug, io::{Seek, self, Write}, fs::File};

pub struct PageFormat {
    header: [u8; FOOTER_HEADER_SIZE],
    slot_cap: u32,
    num_slots: u32,
    slots: [u8; SLOTS_SIZE],
    footer: [u8; FOOTER_HEADER_SIZE]
} 

impl PartialEq for PageFormat {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
        self.num_slots == other.num_slots && 
        self.slot_cap == other.slot_cap && 
        self.slots == other.slots && 
        self.footer == other.footer 
    }
} 

impl Debug for PageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageFormat")
            .field("header", &self.header)
            .field("num_slots", &self.num_slots)
            .field("slot_cap", &self.slot_cap)
            .field("slots", &self.slots)
            .field("footer", &self.footer)
            .finish()
    }
} 

impl PageFormat {
    pub fn new() -> Self {
        let header = [0u8; FOOTER_HEADER_SIZE];
        let footer = [0u8; FOOTER_HEADER_SIZE];

        PageFormat {
            header,
            slot_cap: 10,
            num_slots: 0,
            slots:[0u8; SLOTS_SIZE],
            footer
        }
    } 

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(4096);

        // Header is 16 bytes
        data.extend_from_slice(&self.header);

        // Serialize num_slots
        data.extend_from_slice(&self.num_slots.to_le_bytes());

        // Serialize slot_cap
        data.extend_from_slice(&self.slot_cap.to_le_bytes());

        // Slots is 4096
        data.extend_from_slice(&self.slots);

        // Footer is 16 bytes
        data.extend_from_slice(&self.footer);

        data
    } 

    pub fn deserialize(mut data: Vec<u8>) -> Self {
        // Assumption data vector is 4096 bytes

        // drain first 16 bytes for header
        let header = data.drain(..16).collect();
        let header = array_from_vec::<16>(header);

        // drain next 4 bytes for num slots 
        let num_slots_bytes = data.drain(..4).collect();
        let num_slots = u32::from_le_bytes(array_from_vec::<4>(num_slots_bytes));

        // drain next 4 bytes for slot cap 
        let  slot_cap_bytes = data.drain(..4).collect();
        let slot_cap = u32::from_le_bytes(array_from_vec::<4>(slot_cap_bytes));

        // Drain 4091 bytes for slots 
        let slots = data.drain(..SLOTS_SIZE).collect();
        let slots = array_from_vec::<SLOTS_SIZE>(slots);

        // Reming 16 bytes for footer
        let footer = data.drain(..16).collect();
        let footer = array_from_vec::<16>(footer);

        PageFormat {
            header,
            num_slots,
            slot_cap,
            slots,
            footer,

        } 
    }



    pub fn set_header(&mut self, header: [u8; 16]) {
        self.header = header
    }

    pub fn set_footer(&mut self, footer: [u8; 16]) {
        self.footer = footer
    }

    pub fn get_footer(&self) -> &[u8; 16] {
        &self.footer
    }

    pub fn get_header(&self) -> &[u8; 16] {
        &self.header 
    } 

    pub fn set_slots(&mut self, values: &[u8]) {
        let len = values.len().min(self.slots.len());
        self.slots[..len].copy_from_slice(&values[..len]);
    } 

    pub fn write_to_disk(&self, file: &mut File) -> io::Result<()> {
        file.seek(io::SeekFrom::End(0))?;

        let bytes = self.serialize();

        file.write_all(&bytes)?;
        file.sync_data()?;

        Ok(())
    } 

}

pub fn array_from_vec<const N: usize>(bytes: Vec<u8>) -> [u8; N] {
    bytes[..N].try_into().unwrap() //SIZE would be 16 or 4096
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_serialization() {
        let page = PageFormat::new();

        // Populate page

        let bytes = page.serialize();

        let deserialized = PageFormat::deserialize(bytes);

        assert_eq!(page, deserialized);
    } 

    #[test]
    fn test_read_write_header_footer() {
        let mut page = PageFormat::new();


        // Set new header and footer 
        page.set_header([1u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        page.set_footer([21u8,22,23,24,25,26,27,28,29,210,211,212,213,214,215,216]);

        let header = page.get_header();
        let footer = page.get_footer();

        assert_eq!(page.get_header(), header);
        assert_eq!(page.get_footer(), footer);
    }

    #[test]
    fn test_page_size() {
        let page = PageFormat::new();

        let bytes = page.serialize();

        assert_eq!(bytes.len(), 4096);
    } 

} 
