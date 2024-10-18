#[allow(unused)]
pub const DEFAULT_SCHEMA:&str = "CREATE TABLE sqlite_schema(
  type text,
  name text,
  tbl_name text,
  rootpage integer,
  sql text
);";