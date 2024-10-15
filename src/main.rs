use fish_db::db_mod::db;
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

}

fn try_parsing(database:DB){

}