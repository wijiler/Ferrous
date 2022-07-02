#[derive(Debug)]
pub enum TokenType {
    // Gates
    Or,
    Not,
    XOr,
    And,
    // literals/ident
    Identifier{name:String},
    StringL{value:String},
    CharL{value:String},
    IntNumber{value:String}, 
    FloatNumber{value:String},

    // keywords
    Res_Bool ,
    Res_Int ,
    Res_Uint ,
    Res_Char ,
    Res_String ,
    Res_Float ,
    Res_If ,
    Res_Else ,
    Res_Switch ,
    Res_type ,
    Res_Function ,

    True ,
    False ,

    // symbols and operators
    Bang ,
    Larrow ,
    Rarrow ,
    Main ,
    LParen ,
    Rparen ,
    Equal ,
    Add ,
    Subtract ,
    Multiply ,
    Divide ,
    Modulo ,

    // seperators
    SemiColon ,
    Colon ,
    Comma ,
}
