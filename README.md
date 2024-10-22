# fish_db
A sqlite db parser.

Usage :
cargo run <database file path> <option/command>


If you clone this,

1. Install cargo and rust
2. A test run includes commands like
       . cargo run sample.db .dbinfo (reads db file page header and displays db info)
       . cargo run sample.db .tables (reads the first page of the db file,parses sql_schema table and displays all the tables in the database)
       . cargo run sample.db "select * from apples;" (reads the apples table with just one leaf page right now)
       . cargo run sample.db "select * from oranges;" (reads the oranges table with just one leaf page right now)
