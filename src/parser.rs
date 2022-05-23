use crate::token::TokenType;
/// Types of errors we can get
enum Errort {
    Unknowntoken,
    NoIdentifier,
    NoSemiColon,
}
/// Call an error
fn error(e:Errort,line:usize) {
    match e {
        Errort::NoSemiColon => {
            println!("NoSemiColon at line {}",line);
            return;
         },
        Errort::NoIdentifier => {
            println!("NoIdentifier at line {}",line);
            return;
        },
       Errort::Unknowntoken => {
            println!("Unknowntoken at line {}",line);
            return;
        }
    }
}

enum AsmInstructions {
    movl,
    movq,
    Ascii,
}
