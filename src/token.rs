use std::fs;
use crate::tokentypes::*;
pub enum Token {
    Id(String),       // foo
    Notation(&'static Notate), // 1 + 1
    Int(i64), // 123
    Float(f64), // 1.23
    Uint(u64),
    Literal,  // "s"
    Str,      // "foo"
    Char,     // 's'
    Function(&'static Func), // fn main(args here) { -_- code *-* }
    Return,   // return duh
    Keyword,
}
pub fn tokenize(filename:&String) {
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading please try again");
    // check if file is a ferriclang file
    let mut extension: String = filename.chars().rev().take(2).collect();
    extension = extension.pop().unwrap().to_string() + &extension;
    extension.to_owned();
    if extension != "fr" {
       panic!("not a .fr file please try again"); 
    }
    contents.retain(|c| !c.is_whitespace());
   println!("File contents:\n{}",contents);
}
