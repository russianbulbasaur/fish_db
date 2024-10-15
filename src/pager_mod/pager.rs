use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::db_mod::db::DB;
pub struct Page{
    page_type:u8,
    freeblock_start_address:u16,
    cell_count:u16,
    cell_content_start_address:u16,
    fragmented_free_bytes_count:u8,

    //just in internal nodes
    child_page_pointer:u32
}

impl Page{
    pub fn new(database:&DB,page:u64) -> Page{
        let offset:u64 = u64::from_u8(database.get_page_size())*(page-1);
        let page_size = database.get_page_size();
        let _ = database.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut page_buffer:Vec<u8> = vec![0;u64::from_u8(page_size)];
        database.file.read_exact(&mut page_buffer).expect("Unable to read page");
        let page_type = u8::from_be_bytes([page_buffer[0]]);
        let freeblock_start_address = u16::from_be_bytes([page_buffer[1],page_buffer[2]]);
        let cell_count = u16::from_be_bytes([page_buffer[3],page_buffer[4]]);
        let cell_content_start_address = u16::from_be_bytes([page_buffer[5],page_buffer[6]]);
        let fragmented_free_bytes_count = u8::from_be_bytes([page_buffer[7]]);
        let mut child_page_pointer = 0;
        if page_type==0x0a || page_type==0x0d{
            child_page_pointer = u32::from_be_bytes([
                page_buffer[8],
                page_buffer[9],
                page_buffer[10],
                page_buffer[11],
            ]);
        }
        Page{
            page_type,
            freeblock_start_address,
            cell_count,
            cell_content_start_address,
            fragmented_free_bytes_count,
            child_page_pointer
        }
    }
}