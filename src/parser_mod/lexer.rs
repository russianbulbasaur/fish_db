
#[allow(unused)]
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

    pub fn next_token(&mut self) -> Option<String>{
        let mut formed_token = String::from("");
        if self.pointer== self.chars.len() {
            return None;
        }
        let mut c;
        loop{
            c = self.chars[self.pointer];
            if c==' ' {
                self.pointer += 1;
                if !formed_token.is_empty() {
                    return Some(formed_token);
                }
            }
            if c==';'{
                if !formed_token.is_empty(){
                    return Some(formed_token);
                }
                return None;
            }
            if c==','{
                if !formed_token.is_empty(){
                    return Some(formed_token);
                }
                self.pointer += 1;
                return Some(String::from(","))
            }
            self.pointer += 1;
            formed_token.push(c);
        }
    }
}

