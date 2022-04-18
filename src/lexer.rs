use crate::token::*;


fn main(file:String) {
    struct lexer {
        contents:Vec<char>,
        counter:usize,
    }
    impl lexer {
        pub fn new(c:Vec<char>) -> Self {
           Self {
            contents:c,
            counter:0,
           }
        }
        pub fn lex(&mut self) {
            let clength = self.contents.len();
            let tokens:Vec<Token> = Vec::<Token>::new();
                
            while self.counter < clength {
                   self.counter += 1;
                    match self.getCurrentChar() {

                    } 
            }
        }
                fn getCurrentChar(&self) -> char {
                    *self.contents.get(self.counter).unwrap()
                }
    }
} 
