use crate::pager_mod::pager::{decode_varint};

pub struct TableLeafPage{
}


#[allow(unused)]
pub struct TableLeafPageCell<'a>{
    pub payload:&'a [u8],
    pub row_id:u64,
}


impl TableLeafPage{

    #[allow(unused)]
    pub fn read_cells(content_offset:u8,cell_count:u16,contents:&Vec<u8>) -> Vec<TableLeafPageCell> {
        let mut pointer = content_offset as usize;
        let mut count = 0;
        let mut result = Vec::new();
        while count<cell_count{
            let mut address = u16::from_be_bytes([contents[pointer],
                contents[pointer+1]]) as usize;
            pointer+=2;
            count+=1;
            address -= 100;
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
            let payload = &contents[(address as usize)..(address+payload_size as usize)];
            address += payload_size as usize;
            result.push(TableLeafPageCell{
                payload,
                row_id,
            });
        }
        result
    }
}