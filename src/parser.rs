use crate::lexer::Lexer;
use crate::token::TokenType;


fn parse (tokensa:String) {
let mut lexer = Lexer::new(tokensa);
let tokens = lexer.lex();
}
