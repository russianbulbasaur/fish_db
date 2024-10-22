use regex::Regex;
use crate::parser_mod::lexer::Lexer;

#[allow(unused)]
pub const DEFAULT_SCHEMA:&str = "CREATE TABLE sqlite_schema(
  type text,
  name text,
  tbl_name text,
  rootpage integer,
  sql text
);";


pub struct Parser{

}

impl Parser{
    pub fn new() -> Parser{
        Parser{}
    }

    pub fn parse(&self,source:String) -> String{
        let mut lexer = Lexer::new(source);
        let mut token;
        let mut table_name=String::from("") ;
        loop{
            token = lexer.next_token();
            match token {
                Some(token) => {
                    table_name = token.clone();
                },
                None => break
            }
        }
        table_name
    }

    pub fn parse_columns(&self,sql:String) -> Vec<String>{
        let re = Regex::new(r"(?i)\bCREATE\s+TABLE\s+\w+\s*\(([^)]+)\)").unwrap();
        let mut columns = Vec::new();

        if let Some(captures) = re.captures(sql.as_str()) {
            let columns_str = captures.get(1).unwrap().as_str();

            let column_re = Regex::new(r"(?m)^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s+[a-zA-Z]+").unwrap();

            for cap in column_re.captures_iter(columns_str) {
                columns.push(cap[1].to_string());
            }
        }

        columns
    }
}



