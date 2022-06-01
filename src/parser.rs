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
enum Value {
    string(String),
    int(i64),
    uint(u64),
    float(f64),
}
enum Registers64{
    // temp/return
    rax{value:Value},
    // calee saved
    rbx{value:Value},
    /// rcx rdx rsi rdi r8 r9 are all used to pass function arguments so I will just comment the
    /// number argument they put in
    rcx{value:Value}, // 4
    rdx{value:Value}, // 3
    rsp{value:Value}, // stack pointer
    rbp{value:Value}, // callee saved ; base pointer
    rsi{value:Value}, // 2
    rdi{value:Value}, // 1
    r8{value:Value}, // 5
    r9{value:Value}, // 6
    r10{value:Value}, // temp
    r11{value:Value},// temp
}

//enum Registers32{}

enum AsmInstructions {
  //movl{value:i32,register:Registers32},// this is a 64 bit lang I dont want to work on 32 bit rn
    movq{value:Value,register:Registers64},// 64 bit mov
    ascii{value:String},
}
