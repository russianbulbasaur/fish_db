use crate::pager_mod::pager::{decode_varint};

pub struct IndexLeafPage{
}


#[allow(unused)]
pub struct IndexLeafPageCell<'a>{
    pub payload_size:u64,
    pub payload:&'a [u8],
}


impl IndexLeafPage{

    #[allow(unused)]
    pub fn read_cells(content_offset:u8,cell_count:u16,contents:&Vec<u8>) -> Vec<IndexLeafPageCell> {
        let mut pointer = content_offset as usize;
        let mut result = Vec::new();
        let mut count = 0;
        while count<cell_count{
            let mut address = u16::from_be_bytes([contents[pointer],
                contents[pointer+1]]) as usize;
            let mut decode_result;
            //payload size decode
            decode_result = decode_varint(&contents[address..]);
            let payload_size = decode_result.0;
            address += decode_result.1;

            //payload
            let payload = &contents[(address as usize)..(address+payload_size as usize)];
            address += payload_size as usize;


            pointer += 2;
            count += 1;
            result.push(IndexLeafPageCell{
                payload_size,
                payload
            });
        }
        result
    }
}