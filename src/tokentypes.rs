use crate::token::Token;
struct Args {
tokentype:Token,
id:String,
}

pub struct Func {
    id:String,
    args:Box<Args>,
    contents:String,
}
