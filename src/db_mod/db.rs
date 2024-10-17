use std::fs::File;
use std::io::Read;
use crate::pager_mod::pager::{Page, PageType};
use crate::schema_mod::table::Table;
use crate::pager_mod::table_leaf_page::{TableLeafPage};

#[allow(unused)]
struct Header{
    sql_format:String,
    page_size:u16,
    file_format_write_version:u8,
    file_format_read_version:u8,
    reserved_space_bytes:u8,
    maximum_embedded_payload_fraction:u8,
    min_embedded_payload_fraction:u8,
}

impl Header{
    pub fn get_page_size(&self) -> u16{
        self.page_size
    }
}
#[allow(unused)]
pub struct DB{
    header:Header,
    pub file:File,
    tables:Vec<Table>
}

impl DB{
    pub fn new(path:&str) -> DB {
        let mut db_file = File::open(path).expect("Unable to open file");
        let mut header_buffer = [0;100]; //100 bytes of buffer
        db_file.read_exact(&mut header_buffer).expect("Unable to read header");
        let sql_format = String::from_utf8(header_buffer[0..16].to_owned()).unwrap();
        let page_size = u16::from_be_bytes([header_buffer[16],header_buffer[17]]);
        let header = Header{
            sql_format,
            page_size,
            file_format_write_version: 0,
            file_format_read_version: 0,
            reserved_space_bytes: 0,
            maximum_embedded_payload_fraction: 0,
            min_embedded_payload_fraction: 0,
        };
        let root_page = Page::new_header_page(&mut db_file,page_size);
        #[allow(unused)]
        let data_cells = match root_page.page_type {
            PageType::TableLeafPage => TableLeafPage::read_cells(
                root_page.content_offset,root_page.cell_count,&root_page.contents
            ),
            _ => panic!("Root page should be a leaf page")
        };
        let tables:Vec<Table> = Vec::new();
        DB{
            header,
            file:db_file,
            tables
        }
    }


    pub fn get_page_size(&self) -> u16{
        self.header.get_page_size()
    }
}


