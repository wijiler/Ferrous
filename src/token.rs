enum TokenType {

Ident = 256, // Get used to these numbers because afterlexing the parsers gonna have to parse this

// basic types
Type_String,
Type_Bool,
Type_Int,
Type_Uint,
Type_Char,
}











// old code that would never work
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
/*impl token {
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
    token::new(',');
 }

 fn s_colon() {
    token::new(':');
 }

 fn s_semi() {
    token::new(';');
 }

}
*/
struct Container /*aka function*/ {} //TODO!!!

