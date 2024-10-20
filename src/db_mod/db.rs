use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use crate::pager_mod::pager::{decode_varint, Page, PageType, Pager};
use crate::schema_mod::table::Table;
use crate::pager_mod::table_leaf_page::{TableLeafPage, TableLeafPageCell};
use crate::parser_mod::parser::{Parser, DEFAULT_SCHEMA};
use crate::schema_mod::schema::Schema;

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
    pub tables:Vec<Table>,
    pub parser:Parser,
    pub pager:Pager,
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
        let mut pager = Pager::new(db_file, page_size as u64);
        let root_page = pager.read_root_page();
        #[allow(unused)]
        let data_cells = match root_page.page_type {
            PageType::TableLeafPage => TableLeafPage::read_cells(
                root_page.content_offset,root_page.cell_count,&root_page.contents
            ),
            _ => panic!("Root page should be a leaf page")
        };
        let mut parser = Parser::new();
        let mut tables:Vec<Table> = Vec::new();
        for cell in data_cells{
            let schema:Schema = extract_table(&parser,cell);
            match schema {
                Schema::Table(table) => {
                    tables.push(table);
                },
                _ => {}
            }
        }
        DB{
            header,
            tables,
            parser,
            pager
        }
    }


    pub fn get_page_size(&self) -> u16{
        self.header.get_page_size()
    }
}


fn extract_table(parser: &Parser,data_cell:TableLeafPageCell) -> Schema{
    let mut column_size_store : HashMap<String,u64> = HashMap::new();
    let mut count = 0;
    let mut decode_result = decode_varint(&data_cell.payload[count..]);
    let _payload_header_size = decode_result.0;
    count += decode_result.1;
    let keys:Vec<String> = parser.parse_columns(String::from(DEFAULT_SCHEMA));
    for column_name in &keys{
        decode_result = decode_varint(&data_cell.payload[count..]);
        let data_serial = decode_result.0;
        let data_size = find_size(data_serial);
        count += decode_result.1;
        column_size_store.insert(column_name.clone(),data_size);
    }
    let mut data_store : HashMap<String,Vec<u8>>  = HashMap::new();
    for column_name in keys{
        let data_size = *column_size_store.get(&column_name).expect("");
        data_store.insert(column_name,data_cell.payload[count..(count+data_size as usize)].to_vec());
        count += data_size as usize;
    }
    let schema_type = String::from_utf8(data_store.get("type").unwrap().to_vec()).unwrap();
    let root_page = (data_store.get("rootpage").unwrap().to_vec())[0];
    let sql = String::from_utf8(data_store.get("sql").unwrap().to_vec()).unwrap();
    match schema_type.as_str() {
        "table" =>  Schema::Table(
            Table{
                name: String::from_utf8(data_store.get("tbl_name").unwrap().to_vec()).unwrap(),
                tbl_name: String::from_utf8(data_store.get("tbl_name").unwrap().to_vec()).unwrap(),
                sql:sql.clone(),
                root_page,
                columns:parser.parse_columns(sql),
            }
        ),
        _ => Schema::Index
    }

}

fn find_size(serial:u64) -> u64{
    if serial>=12 && serial%2==0 {
        return (serial-12)/2
    }
    if serial>=13 && serial%2!=0 {
        return (serial-13)/2
    }
    match serial {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 6,
        6 => 8,
        7 => 8,
        8 => 0,
        9 => 0,
        _ => panic!("Size not found for serial {}",serial)
    }
}