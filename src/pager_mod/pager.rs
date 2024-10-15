use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::db_mod::db::DB;
pub struct Page{
    pub page_type:u8,
    pub freeblock_start_address:u16,
    pub cell_count:u16,
    pub cell_content_start_address:u16,
    pub fragmented_free_bytes_count:u8,

    //just in internal nodes
    pub child_page_pointer:u32,
    pub contents:Vec<u8>,
    pub content_offset:u64
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
        let mut content_offset:u64 = 12;
        if page_type==0x0a || page_type==0x0d{
            child_page_pointer = u32::from_be_bytes([
                page_buffer[8],
                page_buffer[9],
                page_buffer[10],
                page_buffer[11],
            ]);
            content_offset = 8;
        }
        Page{
            page_type,
            freeblock_start_address,
            cell_count,
            cell_content_start_address,
            fragmented_free_bytes_count,
            child_page_pointer,
            contents: page_buffer,
            content_offset
        }
    }

    pub fn new_header_page(db_file:&mut File,page_size:u16) -> Page {
        const HEADER_SIZE:i64 = 100;
        let _ = db_file.seek(SeekFrom::Current(HEADER_SIZE)).unwrap();
        let mut page_buffer:Vec<u8> = vec![0;u64::from_u8(page_size)- HEADER_SIZE];
        db_file.read_exact(&mut page_buffer).expect("Unable to read page");
        let page_type = u8::from_be_bytes([page_buffer[0]]);
        let freeblock_start_address = u16::from_be_bytes([page_buffer[1],page_buffer[2]]);
        let cell_count = u16::from_be_bytes([page_buffer[3],page_buffer[4]]);
        let cell_content_start_address = u16::from_be_bytes([page_buffer[5],page_buffer[6]]);
        let fragmented_free_bytes_count = u8::from_be_bytes([page_buffer[7]]);
        let mut child_page_pointer = 0;
        let mut content_offset:u64 = 12;
        if page_type==0x0a || page_type==0x0d{
            child_page_pointer = u32::from_be_bytes([
                page_buffer[8],
                page_buffer[9],
                page_buffer[10],
                page_buffer[11],
            ]);
            content_offset = 8;
        }
        Page{
            page_type,
            freeblock_start_address,
            cell_count,
            cell_content_start_address,
            fragmented_free_bytes_count,
            child_page_pointer,
            contents:page_buffer,
            content_offset
        }
    }

    pub fn read_cells(&self) {
        let mut pointer = self.content_offset;
        let mut count = 0;
        let mut cell_content_addresses = Vec::new();
        while count<self.cell_count{
            cell_content_addresses.push(u16::from_be_bytes([self.contents[pointer],
            self.contents[pointer+1]]));
            pointer += 2;
            count += 1;
        }
    }
}