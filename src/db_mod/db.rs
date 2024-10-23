use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::pager_mod::pager::{decode_varint, Page, PageType, Pager};
use crate::pager_mod::table_interior_page::TableInteriorPage;
use crate::schema_mod::table::Table;
use crate::pager_mod::table_leaf_page::{TableLeafPage, TableLeafPageCell};
use crate::parser_mod::parser::{Parser, Query, DEFAULT_SCHEMA};
use crate::schema_mod::index::Index;
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
                root_page.content_offset,root_page.cell_count,&root_page.contents,
                true
            ),
            _ => panic!("Root page should be a leaf page")
        };
        let parser = Parser::new();
        let mut tables:Vec<Table> = Vec::new();
        for cell in data_cells{
            let schema:Schema = extract_table(&parser,cell);
            match schema {
                Schema::Table(table) => {
                    tables.push(table);
                },
                Schema::Index(index) => {
                    println!("{}",index.name)
                },
                _ => {
                    println!("{:?}",schema);
                }
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

    pub fn execute(&mut self,query:String) {
        let mut parsed_result = self.parser.parse(query);
        for table in &self.tables{
            if table.name==parsed_result.table_name {
                if parsed_result.columns_requested.len()==1 && parsed_result.columns_requested[0]=="*"{
                    parsed_result.columns_requested = table.columns.to_vec();
                }
                self.read_full_table(table.clone(),&parsed_result);
                return;
            }
        }
        println!("no table named {}",parsed_result.table_name);
    }


    pub fn read_full_table(&mut self,table:Table,query:&Query){
        let rootpage = self.pager.read_page(table.root_page as u64);
        let mut result : Vec<HashMap<String,Vec<u8>>> = Vec::new();
        self.read_pages_recursively(rootpage,&mut result,&table,query);
        for map in result{
            let mut printable = Vec::new();
            for col in &query.columns_requested{
               printable.push(String::from_utf8(map.get(col).unwrap().to_vec()).unwrap());
            }
            println!("{}",printable.join("|"));
        }
    }

    fn read_pages_recursively(&mut self,curr_page:Page,
                              result:&mut Vec<HashMap<String,Vec<u8>>>,table:&Table,query:&Query){
        match curr_page.page_type {
            PageType::TableInteriorPage => {
                let data_cells = TableInteriorPage::read_cells(curr_page.content_offset,curr_page.cell_count,
                &curr_page.contents);
                for data_cell in data_cells{
                    let child_page = self.pager.read_page(data_cell.left_child_page_number as u64);
                    self.read_pages_recursively(child_page,result,table,query);
                }
            }
            PageType::TableLeafPage => {
                println!("reading page {}",curr_page.page_number);
                let data_cells = TableLeafPage::read_cells(curr_page.content_offset,
                                                          curr_page.cell_count, &curr_page.contents,false);
                for data_cell in data_cells{
                   let extracted_result = extract_data(&self.parser,
                                                      data_cell,&table);
                    let mut filtered = HashMap::new();
                    for col in &query.columns_requested{
                        filtered.insert(col.clone(),extracted_result.get(col).expect("column not found"));
                    }
                    result.push(extracted_result);
                }
            }
            _ => {}
        }
    }
}


fn extract_data(parser: &Parser,data_cell:TableLeafPageCell,table:&Table) -> HashMap<String,Vec<u8>>{
    let mut column_size_store : HashMap<String,u64> = HashMap::new();
    let mut count = 0;
    let mut decode_result = decode_varint(&data_cell.payload[count..]);
    let _payload_header_size = decode_result.0;
    count += decode_result.1;
    let keys:Vec<String> = parser.parse(table.sql.clone()).columns_requested;
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
    data_store
}


fn extract_table(parser: &Parser,data_cell:TableLeafPageCell) -> Schema{
    let mut column_size_store : HashMap<String,u64> = HashMap::new();
    let mut count = 0;
    let mut decode_result = decode_varint(&data_cell.payload[count..]);
    let _payload_header_size = decode_result.0;
    count += decode_result.1;
    let keys:Vec<String> = parser.parse(String::from(DEFAULT_SCHEMA)).columns_requested;
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
                columns:parser.parse(sql).columns_requested,
            }
        ),
        "index" => Schema::Index(Index{
            name: String::from_utf8(data_store.get("tbl_name").unwrap().to_vec()).unwrap(),
            tbl_name: "".to_string(),
            sql,
            root_page,
            columns: vec![],
        }),
        _ => Schema::View
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