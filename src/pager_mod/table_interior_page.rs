use crate::pager_mod::pager::{decode_varint};

pub struct TableInteriorPage{
}


#[allow(unused)]
pub struct TableInteriorPageCell{
    pub left_child_page_number:u32,
    row_id:u64
}


impl TableInteriorPage{

    #[allow(unused)]
    pub fn read_cells(content_offset:u8,cell_count:u16,contents:&Vec<u8>) -> Vec<TableInteriorPageCell> {
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
            decode_result = decode_varint(&contents[address..]);
            address += decode_result.1;
            let row_id = decode_result.0;
            pointer += 2;
            count += 1;
            result.push(TableInteriorPageCell{
                left_child_page_number,
                row_id
            });
        }
        result
    }
}