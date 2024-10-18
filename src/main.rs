use anyhow::{bail, Result};
use fish_db::db_mod::db::DB;

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
        _ => try_parsing(database)
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

fn try_parsing(_database:DB){

}