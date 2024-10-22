use anyhow::{bail, Result};
use fish_db::db_mod::db::DB;

fn main() -> Result<()> {
    let args:Vec<String> = std::env::args().collect();
    match args.len() {
        1 => bail!("Need database name"),
        2 => bail!("Need a command"),
        _ => {}
    }
    let mut database = DB::new(&args[1]);
    let command = &args[2];
    parse_command(&mut database,command);
    Ok(())
}

fn parse_command(database:& mut DB,command:&str){
    match command {
        ".dbinfo" => show_db_info(database),
        ".tables" => show_tables(database),
        _ => try_parsing(database,command.to_string())
    }
}

fn show_db_info(database:&DB){
    println!("database page size: {}",database.get_page_size());
}


fn show_tables(database:&DB){
    let mut table_names = String::from("");
    for table in &database.tables{
       table_names.push_str(table.name.as_str());
        table_names.push_str(" ");
    }
    println!("{}",table_names);
}

fn try_parsing(database: &mut DB, query:String){
    let parser = &database.parser;
    let table_name = parser.parse(query);
    let mut found_table = false;
    let tables = database.tables.to_owned();
    for table in tables{
        let cloneTable = table.clone();
        if table.name==table_name {
            found_table = true;
            database.read_full_table(cloneTable);
        }
    }
    if !found_table{
        panic!("No table named {}",table_name);
    }
}