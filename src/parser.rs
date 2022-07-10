use crate::lexer::Lexer;
use crate::token::TokenType;
use crate::ast::{astnode::*};

fn parse (tokensa:String) {
let mut lexer = Lexer::new(tokensa);
let mut astvec:Vec<AstNode>;
let tokens = lexer.lex();
    for token in tokens {
        match token {
            TokenType::Add => {} 
            _ => (),
        }
    }
}
