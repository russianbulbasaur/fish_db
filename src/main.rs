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
    for record_pointer in cell_pointers{
        let _ = db_file.seek(SeekFrom::Start(record_pointer as u64));
        let record_size = decode_varint_from_offset(&mut db_file);
        //    println!("record size {}",record_size);
        let row_id = decode_varint_from_offset(&mut db_file);
        //   println!("row id {}",row_id);
        let record_header_size = decode_varint_from_offset(&mut db_file);
        //   println!("record header size {}",record_header_size);
        let (type_serial,type_size) = decode_serial_type_and_size(&mut db_file);
        //  println!("serial for type {} size {}",type_serial,type_size);
        let (name_serial,name_size) = decode_serial_type_and_size(&mut db_file);
        //  println!("serial for name {} size {}",name_serial,name_size);
        let (tbl_name_serial,tbl_name_size) = decode_serial_type_and_size(&mut db_file);
        // println!("serial to tbl_name {} size {}",tbl_name_serial,tbl_name_size);
        let (rootpage_serial,rootpage_size) = decode_serial_type_and_size(&mut db_file);
        // println!("serial for rootpage {} size {}",rootpage_serial,rootpage_size);
        let (sql_serial,sql_size) = decode_serial_type_and_size(&mut db_file);
        //println!("serial for sql {} size {}",sql_serial,sql_size);

        let _ = db_file.seek(SeekFrom::Current(type_size as i64));
        let mut name_buff = [0;1];
        let mut count = 0;
        let mut name_vec = Vec::new();
        while count<name_size{
            db_file.read_exact(&mut name_buff).unwrap();
            count += 1;
            name_vec.push(name_buff[0]);
        }
        let name = String::from_utf8(name_vec).unwrap();
        print!("{} ",name);
    }
}


fn decode_serial_type_and_size(db_file: &mut File) -> (u64, u64){
    let serial_type =  decode_varint_from_offset(db_file);
    let size;
    if serial_type>=12 && serial_type%2==0{
        size = (serial_type-12)/2;
    }else if serial_type>=13 && serial_type%2!=0{
        size = (serial_type-13)/2;
    }else {
        size = 0;
    }
    return (serial_type,size);
}

fn decode_varint_from_offset(db_file: &mut File) -> u64 {
    let mut byte_buffer = [0; 1]; // Buffer to read one byte at a time
    let mut result: u64 = 0;

    loop {
        db_file.read_exact(&mut byte_buffer).unwrap(); // Read a byte
        let byte = byte_buffer[0] & 0b01111111; // Mask the lower 7 bits

        // Shift left by 7 before adding the next byte
        result = (result << 7) | (byte as u64);

        // If the MSB is 0, this is the last byte
        if byte_buffer[0] & 0b10000000 == 0 {
            break;
        }
    }

    result
}
