use crate::schema_mod::index::Index;
use crate::schema_mod::table::Table;

#[allow(unused)]
#[derive(Debug)]
pub enum Schema{
    Table(Table),
    View,
    Trigger,
    Index(Index)
}