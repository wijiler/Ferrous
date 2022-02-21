<<<<<<< HEAD
enum tokentype {
    Id,       // foo
    Notation, // 1 + 1
    Int,      // 123
=======
use std::fs;

pub enum Token {
    Id,       // foo
    Notation, // 1 + 1
    Int(i64), // 123
    Float(f64), // 1.23
    Uint(u64),
>>>>>>> 6ec8c0a (Sectors)
    Literal,  // "s"
    Str,      // "foo"
    Char,     // 's'
    Function, // fn main(args here) { -_- code *-* }
    Return,   // return duh :p
}

<<<<<<< HEAD


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
=======
pub fn tokenize(filename:&String) {
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading please try again");
    contents.retain(|c| !c.is_whitespace());
   println!("File contents:\n{}",contents);
>>>>>>> 6ec8c0a (Sectors)
}
