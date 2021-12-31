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
enum token {
    Ttype(tokentype),
    Structure{}, // depends on the token type see Docs/tokenization.md
}
impl token {
    fn new(tt : tokentype) -> token {
        match tt {
            tokentype::Id => token {
                Ttype(tokentype::Id),
                Structure{
                    Name:String,
                }
            }
        }
    }
}
fn tokenize(file: String) {
    let wssfile = file.replace(" ", "");
    for ct in wssfile.chars().next() {
        if ct == '+' {}
    }
}
