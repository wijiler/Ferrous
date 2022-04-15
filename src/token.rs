/*use std::fs;
pub fn tokenize(filename:&String) {
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading please try again");
    // check if file is a ferriclang file
    let mut extension: String = filename.chars().rev().take(2).collect();
    extension = extension.pop().unwrap().to_string() + &extension;
    if extension != "fr" {
       panic!("not a .fr file please try again"); 
    }
    contents.retain(|c| !c.is_whitespace());
   println!("File contents:\n{}",contents);
}*/
struct token {
    value:char,
}
/*
        TOKEN_ADD,
        TOKEN_SUBTRACT,
        TOKEN_EQUALS,
        TOKEN_BANG,
        TOKEN_MODULO,
        TOKEN_DIVIDE,
        TOKEN_MULTIPLY,
        TOKEN_SEMI,
        TOKEN_LPAREN,
        TOKEN_RPAREN,
        TOKEN_COMMA,
        TOKEN_COLON,
*/
impl token {
 fn new(v:char) -> Self {
    Self {value:v,}
 }
 // o for operator etc. etc.
 fn o_equal() {
    token::new('=');
 }

 fn o_minus() {
    token::new('-');
 }

 fn o_add() {
    token::new('+');
 }

 fn o_divide() {
    token::new('/');
 }

 fn o_multiply() {
    token::new('*');
 }

 fn t_lparen() {
    token::new('(');
 }

 fn t_rparen() {
    token::new(')');
 }

 fn s_comma() {
    token::new(':');
 }

 fn s_colon() {
    token::new(';');
 }

 fn s_semi() {
    token::new(';');
 }

}
