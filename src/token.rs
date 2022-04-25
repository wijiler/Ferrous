#[derive(Debug)]
pub enum TokenType {
// Gates
Or = 29, 
Not = 30, // too lazy to delete just wont be used replaced by Bang
XOr = 31,
And = 32,


Identifier = 0,
STRING = 1,
Int = 2,


// keywords
Res_Bool = 5,
Res_Int = 6,
Res_Uint = 7,
Res_Char = 8,
Res_String = 9,
Res_Float = 28,
Res_If = 24,
Res_Else = 26,
Res_Switch = 25,
Res_type = 27,
Res_Function = 35,

True = 33,
False = 34,

// symbols and operators
Bang = 10,
Larrow = 11,
Rarrow = 12,
Main = 36,
LParen = 13,
Rparen = 14,
Equal = 15,
Add = 16,
Subtract = 17,
Multiply = 18,
Divide = 19,
Modulo = 20,

// seperators
SemiColon = 21,
Colon = 22,
Comma=23,

}
/*
pub enum Tokenvalue {
    Boolval(bool),    
    Intval(i64),    
    Floatval(f64),    
    UIntval(u64),
    CharVal(char),
    StringVal{count:u64, data:String}, // make it easier to get the length
    FunctionVal{args:[&'static Tokenvalue;256], contents:TokenType},
    IfVal{condition:bool,doesoncondition:TokenType},
    ElseVal{does:TokenType},
    Operator{},
    Identifier(String),
}
*/
#[derive(Debug)]
pub struct Token {
    Type:TokenType,
    as_literal:String,
}
impl Token {
pub fn new(t:TokenType,a:String) -> Self {
            Self {
                Type:t,
                as_literal:a,
            }
    }
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
