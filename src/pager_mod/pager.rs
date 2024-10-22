use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::db_mod::db::DB;
use crate::pager_mod::table_interior_page::TableInteriorPage;
use crate::parser_mod::parser::Parser;

#[allow(unused)]
pub struct Page {
    pub page_number:u64,
    pub freeblock_start_address:u16,
    pub cell_count:u16,
    pub cell_content_start_address:u16,
    pub fragmented_free_bytes_count:u8,

    //just in internal nodes
    pub child_page_pointer:u32,
    pub contents:Vec<u8>,
    pub content_offset:u8,
    pub page_type: PageType
}

#[derive(Debug)]
pub enum PageType{
    TableInteriorPage,
    TableLeafPage,
    IndexLeafPage,
    IndexInteriorPage
}

pub struct Pager{
    db_file:File,
    page_size:u64
}

impl Pager{
    pub fn new(db_file:File,page_size:u64) -> Pager{
        Pager{
            page_size,
            db_file
        }
    }
    
    pub fn read_page(&mut self, page:u64) -> Page{
        let offset:u64 = (self.page_size * (page - 1)) as u64;
        let page_size = self.page_size as usize;
        let _ = self.db_file.seek(SeekFrom::Start(offset)).unwrap();
        let mut page_buffer:Vec<u8> = vec![0; page_size];
        self.db_file.read_exact(&mut page_buffer).expect("Unable to read page");
        let page_type_byte = u8::from_be_bytes([page_buffer[0]]);
        let freeblock_start_address = u16::from_be_bytes([page_buffer[1],page_buffer[2]]);
        let cell_count = u16::from_be_bytes([page_buffer[3],page_buffer[4]]);
        let cell_content_start_address = u16::from_be_bytes([page_buffer[5],page_buffer[6]]);
        let fragmented_free_bytes_count = u8::from_be_bytes([page_buffer[7]]);
        let mut child_page_pointer = 0;
        let mut content_offset = 12;
        let page_type:PageType;
        if page_type_byte==0x0a || page_type_byte==0x0d{
            child_page_pointer = u32::from_be_bytes([
                page_buffer[8],
                page_buffer[9],
                page_buffer[10],
                page_buffer[11],
            ]);
            content_offset = 8;
        }
        match page_type_byte {
            0x0a => page_type = PageType::IndexLeafPage,
            0x0d => page_type = PageType::TableLeafPage,
            0x05 => page_type = PageType::TableInteriorPage,
            0x02 => page_type = PageType::IndexInteriorPage,
            _ => panic!("Unrecognized page")
        }
        Page{
            page_number:page,
            freeblock_start_address,
            cell_count,
            cell_content_start_address,
            fragmented_free_bytes_count,
            child_page_pointer,
            contents: page_buffer,
            content_offset,
            page_type
        }
    }
    
    pub fn read_root_page(&mut self) -> Page{
        const HEADER_SIZE:usize = 100;
        let _ = self.db_file.seek(SeekFrom::Start(HEADER_SIZE as u64)).unwrap();
        let mut page_buffer:Vec<u8> = vec![0;self.page_size as usize - HEADER_SIZE];
        self.db_file.read_exact(&mut page_buffer).expect("Unable to read page");
        let page_type_byte = u8::from_be_bytes([page_buffer[0]]);
        let freeblock_start_address = u16::from_be_bytes([page_buffer[1],page_buffer[2]]);
        let cell_count = u16::from_be_bytes([page_buffer[3],page_buffer[4]]);
        let cell_content_start_address = u16::from_be_bytes([page_buffer[5],page_buffer[6]]);
        let fragmented_free_bytes_count = u8::from_be_bytes([page_buffer[7]]);
        let mut child_page_pointer = 0;
        let mut content_offset = 12;
        let page_type:PageType;
        match page_type_byte {
            0x0a => page_type = PageType::IndexLeafPage,
            0x0d => page_type = PageType::TableLeafPage,
            0x05 => page_type = PageType::TableInteriorPage,
            0x02 => page_type = PageType::IndexInteriorPage,
            _ => panic!("Unrecognized page")
        }
        if page_type_byte==0x0a || page_type_byte==0x0d{
            child_page_pointer = u32::from_be_bytes([
                page_buffer[8],
                page_buffer[9],
                page_buffer[10],
                page_buffer[11],
            ]);
            content_offset = 8;
        }
        Page{
            page_number:1,
            freeblock_start_address,
            cell_count,
            cell_content_start_address,
            fragmented_free_bytes_count,
            child_page_pointer,
            contents:page_buffer,
            content_offset,
            page_type
        }
    }
}



pub fn decode_varint(bytes: &[u8]) -> (u64,usize) {
    let mut result: u64 = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        result = (result << 7) | (byte & 0x7F) as u64;
        if byte & 0x80 == 0 {
            return (result,i+1);
        }
        if i == 9 {
            panic!("Varint longer than 10 bytes")
        }
    }
    panic!("Decode varint error")
}