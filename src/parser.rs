use std::fs;
use crate::token::TokenType;
use aho_corasick::AhoCorasick;
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
    // r12-15 are all callee registers
    r12{value:Value},
    r13{value:Value},
    r14{value:Value},
    r15{value:Value},
}
enum fRegister {
    xmm0{value:f64},
    xmm1{value:f64},
    xmm2{value:f64},
    xmm3{value:f64},
    xmm4{value:f64},
    xmm5{value:f64},
    xmm6{value:f64},
    xmm7{value:f64},
}
enum Value {
    string(String),
    int(i64),
    uint(u64),
    fregister(fRegister),
    register(Box<Registers64>),
}
enum AsmInstructions {
  //movl{value:i32,register:Registers32},// this is a 64 bit lang I dont want to work on 32 bit rn
    movq{value:Value,register:Registers64},// 64 bit mov
    ascii{value:String},
    call{label:String},
    addq{a:Value,b:Value},
    subq{a:Value,b:Value},
    imulq{a:Value,b:Value},
    idivq{a:Value,b:Value},
    xor{a:Value,b:Value},
}
struct AstNode {
    value:String,
}
impl AstNode {
    fn new(v:String) -> Self {
        Self {
            value:v,
        }
    }
}
pub fn create_ast () {
}
fn get_nearest_Type () {

}
