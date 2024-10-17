use crate::pager_mod::pager::{decode_varint};

pub struct TableLeafPage{
}

pub struct TableLeafPageCell{
}


impl TableLeafPage{

    #[allow(unused)]
    pub fn read_cells(content_offset:u8,cell_count:u16,contents:&Vec<u8>) -> Vec<TableLeafPageCell> {
        let mut pointer = content_offset as usize;
        let mut count = 0;
        while count<cell_count{
            let mut address = u16::from_be_bytes([contents[pointer],
                contents[pointer+1]]) as usize;
            let mut decode_result;
            //payload size decode
            decode_result = decode_varint(&contents[address..]);
            let payload_size = decode_result.0;
            address += decode_result.1;

            //row id
            decode_result = decode_varint(&contents[address..]);
            let row_id = decode_result.0;
            address += decode_result.1;

            //payload
            let payload = &contents[(address as usize)..(address+payload_size as usize+1)];
            address += payload_size as usize + 1;
            //page number of overflow page
            let overflow_page_number = u32::from_be_bytes([
                contents[address],
                contents[address+1],
                contents[address+2],
                contents[address+3],
            ]);
            pointer += 2;
            count += 1;
            println!("{}",payload_size)
        }
        let result = Vec::new();
        result
    }
}