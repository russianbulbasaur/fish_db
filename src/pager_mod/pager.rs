use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::db_mod::db::DB;
use crate::pager_mod::index_interior_page::IndexInteriorPage;
use crate::pager_mod::index_leaf_page::IndexLeafPage;
use crate::pager_mod::table_interior_page::TableInteriorPage;
use crate::pager_mod::table_leaf_page::TableLeafPage;

#[allow(unused)]
pub struct Page<'a> {
    pub page_type_byte:u8,
    pub freeblock_start_address:u16,
    pub cell_count:u16,
    pub cell_content_start_address:u16,
    pub fragmented_free_bytes_count:u8,

    //just in internal nodes
    pub child_page_pointer:u32,
    pub contents:Vec<u8>,
    pub content_offset:u8,
    pub page_type: PageType<'a>
}

enum PageType<'a>{
    TableInteriorPage(TableInteriorPage<'a>),
    TableLeafPage(TableLeafPage<'a>),
    IndexLeafPage(IndexLeafPage<'a>),
    IndexInteriorPage(IndexInteriorPage<'a>)
}

impl PageType{
    pub fn read_cells<T>(&self) -> T{
        match &self {
            PageType::TableInteriorPage(page_type) => {
            }
            PageType::TableLeafPage(page_type) => {
                page_type.read_cells();
            }
            PageType::IndexLeafPage(page_type) => {}
            PageType::IndexInteriorPage(page_type) => {}
        }
    }
}


#[allow(unused)]
impl Page{
    pub fn new(database: &mut DB, page:u64) -> Page {
        let offset:u64 = (database.get_page_size() as u64 * (page - 1)) as u64;
        let page_size = database.get_page_size();
        let _ = database.file.seek(SeekFrom::Start(offset)).unwrap();
        let mut page_buffer:Vec<u8> = vec![0; page_size as usize];
        database.file.read_exact(&mut page_buffer).expect("Unable to read page");
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
            0x0a => page_type = PageType::IndexLeafPage(IndexLeafPage::new(
                &page_buffer,content_offset,cell_count
            )),
            0x0d => page_type = PageType::TableLeafPage(TableLeafPage::new(
                &page_buffer,content_offset,cell_count
            )),
            0x05 => page_type = PageType::TableInteriorPage(TableInteriorPage::new(
                &page_buffer,content_offset,cell_count
            )),
            0x02 => page_type = PageType::IndexInteriorPage(IndexInteriorPage::new(
                &page_buffer,content_offset,cell_count
            )),
            _ => panic!("Unrecognized page")
        }
        Page{
            page_type_byte,
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

    pub fn new_header_page(db_file:&mut File,page_size:u16) -> Page {
        const HEADER_SIZE:usize = 100;
        let _ = db_file.seek(SeekFrom::Start(HEADER_SIZE as u64)).unwrap();
        let mut page_buffer:Vec<u8> = vec![0;(page_size as usize) - HEADER_SIZE];
        db_file.read_exact(&mut page_buffer).expect("Unable to read page");
        let page_type_byte = u8::from_be_bytes([page_buffer[0]]);
        let freeblock_start_address = u16::from_be_bytes([page_buffer[1],page_buffer[2]]);
        let cell_count = u16::from_be_bytes([page_buffer[3],page_buffer[4]]);
        let cell_content_start_address = u16::from_be_bytes([page_buffer[5],page_buffer[6]]);
        let fragmented_free_bytes_count = u8::from_be_bytes([page_buffer[7]]);
        let mut child_page_pointer = 0;
        let mut content_offset = 12;
        let page_type:PageType;
        match page_type_byte {
            0x0a => page_type = PageType::IndexLeafPage(IndexLeafPage::new(
                &page_buffer,content_offset,cell_count
            )),
            0x0d => page_type = PageType::TableLeafPage(TableLeafPage::new(
                &page_buffer,content_offset,cell_count
            )),
            0x05 => page_type = PageType::TableInteriorPage(TableInteriorPage::new(
                &page_buffer,content_offset,cell_count
            )),
            0x02 => page_type = PageType::IndexInteriorPage(IndexInteriorPage::new(
                &page_buffer,content_offset,cell_count
            )),
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
            page_type_byte,
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

fn decode_varint(bytes: &[u8]) -> Option<u64> {
    let mut result: u64 = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        result = (result << 7) | (byte & 0x7F) as u64;
        if byte & 0x80 == 0 {
            return Some(result);
        }
        if i == 9 {  // Varints longer than 10 bytes would overflow a u64
            return None;
        }
    }
    None  // Input was truncated
}