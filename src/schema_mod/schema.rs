use crate::schema_mod::table::Table;

#[allow(unused)]
pub enum Schema{
    Table(Table),
    View,
    Trigger,
    Index
}