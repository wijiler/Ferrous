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
//pub fn get_ident_by_index(array:&Vec<TokenType>,index:usize) -> String {
//        let ident:String;
//        match &array[index] {
//            TokenType::Identifier{name} => {
//                ident = name.to_string();
//                return ident;
//            }
//            _ => { panic!("I wrote the compiler wrong and there is no identifier here") },
//        }
//}
fn genast(ast:Vec<AstNode>,tokensa:String) {
   let mut lexer = Lexer::new(tokensa);
   let tokens = lexer.lex();
   let mut main:AstNode;
   for (mut index,token) in tokens.iter().enumerate() {
       match token {
           TokenType::Main => {
           },

           _ => (),
       }
   }
}
