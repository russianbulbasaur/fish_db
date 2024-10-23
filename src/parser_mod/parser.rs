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
        let select_re = Regex::new(r"(?i)\bSELECT\s+([^;]+)\s+FROM\s+([^\s;]+)(?:\s+WHERE\s+(.+?))?;?").unwrap();

        let mut query = Query {
            table_name: "".to_string(),
            columns_requested: vec![],
            where_clauses: Default::default(),
        };

        // Try to match CREATE TABLE statement
        if let Some(captures) = create_re.captures(source.as_str()) {
            // Extract table name
            query.table_name = captures.get(1).map_or_else(|| "".to_string(), |m| m.as_str().to_string());
            let columns_str = captures.get(2).unwrap().as_str();

            // Updated regex to correctly capture columns, ignoring constraints
            let column_re = Regex::new(r"\s*([^,\s]+)\s+([^\s,]+)(?:\s+[^,\s]+)*").unwrap();

            for cap in column_re.captures_iter(columns_str) {
                // Capture column name only
                let column_name = &cap[1];
                query.columns_requested.push(column_name.to_string());
            }
        }
        // Try to match SELECT statement
        else if let Some(captures) = select_re.captures(source.as_str()) {
            // Extract columns
            let columns_str = captures.get(1).unwrap().as_str();
            let columns_vec: Vec<&str> = columns_str.split(',').map(|s| s.trim()).collect();
            query.columns_requested.extend(columns_vec.iter().map(|s| s.to_string()));

            // Extract table name
            query.table_name = captures.get(2).map_or_else(|| "".to_string(), |m| m.as_str().to_string());

            // Extract where clauses
            if let Some(where_clause) = captures.get(3) {
                let conditions = where_clause.as_str();
                let condition_re = Regex::new(r"(\w+)\s*=\s*'([^']*)'").unwrap();
                for cap in condition_re.captures_iter(conditions) {
                    query.where_clauses.insert(cap[1].to_string(), cap[2].to_string());
                }
            }
        } else {
            // Return default query struct on error
            return query;
        }

        query
    }
}



