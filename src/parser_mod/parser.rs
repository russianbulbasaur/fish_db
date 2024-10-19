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

    pub fn parse(&self,source:String){
        let mut lexer = Lexer::new(source);
        let mut token;
        loop{
            token = lexer.next_token();
            match token {
                Some(token) => {
                    println!("{:?}",token.token_type);
                    println!("{:?}",token.arguments);
                },
                None => break
            }
        }
        println!("End of parsing")
    }
}