#[derive(Debug)]
pub struct Table{
    pub name:String,
    pub tbl_name:String,
    pub sql:String,
    pub root_page:u8,
    pub columns:Vec<String>
}

impl Table{
    pub fn read_table(){

    }
}


impl Clone for Table{
    fn clone(&self) -> Self {
        Table{
            name: self.name.to_owned(),
            tbl_name: self.tbl_name.to_owned(),
            sql: self.sql.to_owned(),
            root_page: self.root_page,
            columns: self.columns.to_owned(),
        }
    }

    fn clone_from(&mut self, _source: &Self) {
        //not implied
    }
}