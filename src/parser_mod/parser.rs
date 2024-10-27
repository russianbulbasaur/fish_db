use std::collections::HashMap;
use regex::Regex;

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


pub struct Query{
    pub table_name:String,
    pub columns_requested:Vec<String>,
    pub where_clauses:HashMap<String,String>
}

impl Parser{
    pub fn new() -> Parser{
        Parser{}
    }

    pub fn parse(&self, source: String) -> Query {
        let create_re = Regex::new(r"(?i)\bCREATE\s+TABLE\s+([^\s(]+)\s*\(([^)]+)\)").unwrap();
        let select_re = Regex::new(r"(?i)\bSELECT\s+([^;]+)\s+FROM\s+([^\s;]+)(?:\s+WHERE\s+(.+))?;?").unwrap();
        let mut query = Query {
            table_name: "".to_string(),
            columns_requested: vec![],
            where_clauses: Default::default(),
        };

        if let Some(captures) = create_re.captures(source.as_str()) {
            query.table_name = captures.get(1).map_or_else(|| "".to_string(), |m| m.as_str().to_string());
            let columns_str = captures.get(2).unwrap().as_str();
            let column_re = Regex::new(r"\s*([^,\s]+)\s+([^\s,]+)(?:\s+[^,\s]+)*").unwrap();
            for cap in column_re.captures_iter(columns_str) {
                let column_name = &cap[1];
                query.columns_requested.push(column_name.to_string());
            }
        }
        else if let Some(captures) = select_re.captures(source.as_str()) {
            let columns_str = captures.get(1).unwrap().as_str();
            let columns_vec: Vec<&str> = columns_str.split(',').map(|s| s.trim()).collect();
            query.columns_requested.extend(columns_vec.iter().map(|s| s.to_string()));
            query.table_name = captures.get(2).map_or_else(|| "".to_string(), |m| m.as_str().to_string());
            if let Some(where_clause) = captures.get(3) {
                let conditions = where_clause.as_str();
                let condition_re = Regex::new(r"(?i)(\w+)\s*=\s*'?([^' ]+)'?").unwrap();
                for cap in condition_re.captures_iter(conditions) {
                    query.where_clauses.insert(cap[1].to_string(), cap[2].to_string());
                }
            }
        }
        query
    }


    pub fn extract_columns_from_index(&self,query: String) -> Vec<String> {
        let re = Regex::new(r"(?i)create\s+index\s+\w+\s+on\s+\w+\s*\(([^)]+)\)").unwrap();
        if let Some(captures) = re.captures(query.as_str()) {
            if let Some(columns) = captures.get(1) {
                return columns.as_str()
                    .split(',')
                    .map(|col| col.trim().to_string())
                    .collect();
            }
        }
        Vec::new()
    }
}



