use crate::pager_mod::pager::Page;

pub struct TableLeafPage<'a>{
    pub contents:&'a Vec<u8>,
    pub content_offset:u8,
    pub cell_count:u16,
}

struct TableLeafPageCell{
}


impl TableLeafPage{
    pub fn new(contents:&Vec<u8>,content_offset:u8,cell_count:u16) -> TableLeafPage{
        TableLeafPage{
            contents,
            content_offset,
            cell_count
        }
    }

    pub fn read_cells(&self) -> Vec<TableLeafPageCell> {
        let mut pointer = self.content_offset as usize;
        let mut count = 0;
        let mut cell_content_addresses = Vec::new();
        while count<self.cell_count{
            let address = u16::from_be_bytes([self.contents[pointer],
                self.contents[pointer+1]]);
            pointer += 2;
            count += 1;
        }
        let mut result = Vec::new();
        result
    }
}