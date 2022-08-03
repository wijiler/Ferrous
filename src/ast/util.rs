use token::TokenType;
#[derive(Debug,Clone)]
pub struct Parser {
   tokens:Vec<TokenType>, 
}

impl Parser {
    pub fn new (t:Vec<TokenType>) -> Self {
        Self {
            tokens:t
        }
    }
    pub fn peek(&self) -> TokenType {
        self.tokens.to_vec().into_iter().next().unwrap()
    }  
    pub fn nom (self) -> TokenType { // its the eat/consume function but I think its funnier to call it nom
        self.tokens.to_vec().into_iter().next().unwrap()
    }
}
