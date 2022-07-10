use crate::token::TokenType;
pub fn get_ident_by_index(array:&Vec<TokenType>,index:usize) -> String {
        let ident:String;
        match &array[index] {
            TokenType::Identifier{name} => {
                ident = name.to_string();
                return ident;
            }
            _ => { panic!("I wrote the compiler wrong and there is no identifier here") },
        }
}
