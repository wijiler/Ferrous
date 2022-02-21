use token::Token;
struct Args {
tokentype:Token,
id:String,
}

pub struct Func {
    id:String,
    args:Args,
    contents:String,
}
