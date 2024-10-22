# fish_db
A sqlite db parser.

Usage :
cargo run <database file path> <option/command>


If you clone this,

1. Install cargo and rust
2. A test run includes commands like
   1. cargo run sample.db .dbinfo (reads db file page header and displays db info)
   2. cargo run sample.db .tables (reads the first page of the db file,parses sql_schema table and displays all the tables in the database)
   3. cargo run sample.db "select * from apples;" (reads the apples table with just one leaf page right now)
   4. cargo run sample.db "select * from oranges;" (reads the oranges table with just one leaf page right now)
