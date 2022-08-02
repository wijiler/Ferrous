//use crate::parserutil::expression;
use crate::token::TokenType;
#[derive(Debug,Clone)]
pub struct Parser {
   tokens:Vec<TokenType>, 
}

impl Parser {
    pub fn peek(&mut self) -> TokenType {
        self.tokens.to_owned().into_iter().next().unwrap()
    }  
}
fn parse (p: &mut Parser){
    p.peek();
}
