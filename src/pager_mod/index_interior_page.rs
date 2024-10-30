use crate::pager_mod::pager::{decode_varint};

pub struct IndexInteriorPage{
}


#[allow(unused)]
pub struct IndexInteriorPageCell<'a>{
    pub left_child_page_number:u32,
    pub payload_size:u64,
    pub payload:&'a [u8],
}


impl IndexInteriorPage{

    #[allow(unused)]
    pub fn read_cells(content_offset:u8,cell_count:u16,contents:&Vec<u8>) -> Vec<IndexInteriorPageCell> {
        let mut pointer = content_offset as usize;
        let mut result = Vec::new();
        let mut count = 0;
        while count<cell_count{
            let mut address = u16::from_be_bytes([contents[pointer],
                contents[pointer+1]]) as usize;
            let mut decode_result;
            let left_child_page_number = u32::from_be_bytes([
                contents[address],
                contents[address+1],
                contents[address+2],
                contents[address+3]
            ]);
            address += 4;
            //payload size decode
            decode_result = decode_varint(&contents[address..]);
            let payload_size = decode_result.0;
            address += decode_result.1;

            //payload
            let payload = &contents[(address as usize)..(address+payload_size as usize)];
            address += payload_size as usize;



            pointer += 2;
            count += 1;
            result.push(IndexInteriorPageCell{
                left_child_page_number,
                payload_size,
                payload,
            });
        }
        result
    }
}