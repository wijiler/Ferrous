enum tokentype {
    Id,       // foo
    Notation, // 1 + 1
    Int,      // 123
    Literal,  // "s"
    Str,      // "foo"
    Char,     // 's'
    Function, // fn main(args here) { -_- code *-* }
    Return,   // return duh :p
}



enum structures {
    Id{
    Name:String,
    },
    Notation{
    a: i128,
    operation:Option<char>,
    b: i128,
    },
}

struct token {
    ttype: Option<tokentype>,
    structure:, // depends on the token type see Docs/tokenization.md
}
impl token {
    fn new(tt : tokentype) -> token {
    }
}
fn tokenize(file: String) {
    let wssfile = file.replace(" ", "");
    for ct in wssfile.chars().next() { 
        if ct == '+' {}
    }
}
