use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }
    // Parse command
    return parse_command(&args[2], &args[1]);
}


fn parse_command(command: &String, filename:&String) -> Result<()>{
    match command.as_str() {
        ".dbinfo" => {
            db_info(filename);
        }
        ".tables" => {
            tables(filename);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }
    Ok(())
}

fn db_info(filename:&String){
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("{}",err)
    };
    let mut header = [0; 100]; //first page
    match file.read_exact(&mut header){
        Ok(_) => "",
        Err(err) => panic!("{}",err)
    };
    //page size
    #[allow(unused_variables)]
    let page_size = u16::from_be_bytes([header[16], header[17]]);
    println!("database page size: {}", page_size);

    //table number at 2 bytes after header at offset 3
    let mut btree_page_header = [0;5];
    file.read_exact(&mut btree_page_header).expect("Unable to read table count bytes");
    let table_num = u16::from_be_bytes([btree_page_header[3], btree_page_header[4]]);
    println!("number of tables: {}", table_num);
}

fn tables(filename:&String){
    let mut db_file = match File::open(filename){
        Ok(file) => file,
        Err(err) => panic!("{}",err)
    };

    let _ = match db_file.seek(SeekFrom::Start(100)){
        Ok(offset) => offset,
        Err(err) => panic!("{}",err)
    };
    let mut btree_header_part = [0;5];
    db_file.read_exact(&mut btree_header_part).unwrap();
    let page_type = u8::from_be_bytes([btree_header_part[0]]);

    let mut table_num = u16::from_be_bytes([btree_header_part[3], btree_header_part[4]]);
    println!("number of tables: {}", table_num);

    if page_type==2 || page_type==5{
        //12 bytes ka page header
        let _ = db_file.seek(SeekFrom::Current(7));
    }else if page_type==10 || page_type==13{
        let _ = db_file.seek(SeekFrom::Current(3));
    }
    let mut cell_pointers:Vec<u16> = Vec::new();
    while table_num > 0{
        let mut buff= [0;2];
        db_file.read_exact(&mut buff).expect("TODO: panic message");
        cell_pointers.push(u16::from_be_bytes(buff));
        table_num -= 1;
    }
    println!("{}\n{}\n{}",cell_pointers[0],cell_pointers[1],cell_pointers[2]);
}