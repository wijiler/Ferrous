use ast::util::Parser;
use crate::token::TokenType;
pub fn literal (literaltype:TokenType,tokens:Vec<TokenType>) {
    let parser:Parser = Parser::new(tokens);
    match literaltype {
        TokenType::IntNumber{..} => {
             
        }
        _ => ()
    }
}
