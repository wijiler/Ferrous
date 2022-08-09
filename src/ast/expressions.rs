use ast::util::*;
use crate::token::TokenType;
struct Expressions { tokens:Vec<TokenType>  }
impl Expressions {
pub fn literal (&self,literaltype:TokenType,int:Option<i64>,uint:Option<u64>,string:Option<String>) { 
    match literaltype {
        TokenType::IntNumber{..} => {
             let ident = self.tokens.to_vec().into_iter().next_back(); 
             if let TokenType::Identifier(_) = ident.unwrap(){
                
             }
        }
        _ => ()
    }
}
}
