use super::astnode::{AstNode,NodeType};
use crate::token::TokenType;
use crate::lexer::Lexer;
struct Ast {
    root:AstNode,
    children:Vec<AstNode>,
}
impl Ast {
    pub fn new(root:AstNode,children:Vec<AstNode>) -> Self {
           Self {
               root:root,
               children:children,
           }
    }
}

fn genast(ast:Vec<AstNode>,tokensa:String) {
   let mut lexer = Lexer::new(tokensa);
   let tokens = lexer.lex();
}
