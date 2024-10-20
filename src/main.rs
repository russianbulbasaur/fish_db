use anyhow::{bail, Result};
use fish_db::db_mod::db::DB;
use fish_db::pager_mod::table_interior_page::TableInteriorPage;
use fish_db::pager_mod::table_interior_page::TableInteriorPageCell;

fn main() -> Result<()> {
    let args:Vec<String> = std::env::args().collect();
    match args.len() {
        1 => bail!("Need database name"),
        2 => bail!("Need a command"),
        _ => {}
    }
    let database = DB::new(&args[1]);
    let command = &args[2];
    parse_command(database,command);
    Ok(())
}

fn parse_command(database:DB,command:&str){
    match command {
        ".dbinfo" => show_db_info(database),
        ".tables" => show_tables(database),
        _ => try_parsing(database,command.to_string())
    }
}

fn show_db_info(database:DB){
    println!("database page size: {}",database.get_page_size());
}


fn show_tables(database:DB){
    let mut table_names = String::from("");
    for table in database.tables{
       table_names.push_str(table.name.as_str());
        table_names.push_str(" ");
    }
    println!("{}",table_names);
}

fn try_parsing(mut database:DB, query:String){
    let parser = database.parser;
    let table_name = parser.parse(query);
    let mut found_table = false;
    for table in database.tables{
        if table.name==table_name {
            found_table = true;
            let page = database.pager.read_page(table.root_page as u64);
        }
    }
    if !found_table{
        panic!("No table named {}",table_name);
    }
}