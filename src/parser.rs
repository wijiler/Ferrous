use crate::lexer::Lexer;

fn parse (file:String) {
let mut lexer = Lexer::new(file);
let lexed = lexer.lex();
}
