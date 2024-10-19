

pub struct Lexer{
    chars:Vec<char>,
    pointer:usize,
    source:String
}

impl Lexer{
    pub fn new(source:String) -> Lexer{
        Lexer{
            chars:source.chars().collect(),
            pointer:0,
            source
        }
    }

    pub fn next_token(&mut self) -> Option<Token>{
        let mut formed_token = String::from("");
        if self.pointer== self.chars.len() {
            return None;
        }
        let mut c;
        loop{
            c = self.chars[self.pointer];
            self.pointer += 1;
            if c==' ' {
                break;
            }
            formed_token.push(c);
        }
        match formed_token.as_str() {
            "FROM" =>{
                let arguments = self.parse_from_arguments();
                match arguments {
                    Some(args) => {
                        Some(Token{
                            token_type:TokenType::FROM,
                            arguments:Some(args)
                        })
                    },
                    None => panic!("Arguments to FROM expected")
                }
            },
            "SELECT" => {
                let arguments = self.parse_select_arguments();
                match arguments {
                    Some(args) => {
                        Some(Token{
                            token_type:TokenType::SELECT,
                            arguments:Some(args)
                        })
                    },
                    None => panic!("Arguments to FROM expected")
                }
            },
            _ => panic!("unknown token type at {} in {}",self.pointer+1,self.source)
        }
    }

    fn parse_from_arguments(&mut self) -> Option<Vec<String>>{
        let mut args = Vec::new();
        let mut c;
        let mut formed_token = String::from("");
        loop{
            c = self.chars[self.pointer];
            self.pointer += 1;
            if c==' ' || c==';'{
                args.push(formed_token.clone());
                formed_token.clear();
                break;
            }
            if c==','{
                args.push(formed_token.clone());
                formed_token.clear();
                continue;
            }
            formed_token.push(c);
        }
        if args.is_empty() {
            return None
        }
        Some(args)
    }

    fn parse_select_arguments(&mut self) -> Option<Vec<String>>{
        let mut args = Vec::new();
        let mut c;
        let mut formed_token = String::from("");
        loop{
            c = self.chars[self.pointer];
            self.pointer += 1;
            if c==' ' || c==';'{
                args.push(formed_token.clone());
                formed_token.clear();
                break;
            }
            if c==','{
                args.push(formed_token.clone());
                formed_token.clear();
                continue;
            }
            formed_token.push(c);
        }
        if args.is_empty() {
            return None
        }
        Some(args)
    }
}


#[derive(Debug)]
#[allow(unused)]
pub enum TokenType{
    CREATE,
    SELECT,
    FROM,
    COUNT,
    BracketStart,
    BracketEnd,
}

pub struct Token{
    pub token_type:TokenType,
    pub arguments:Option<Vec<String>>
}
