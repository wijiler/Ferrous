use crate::token::*;
use std::{fs::File, io::Write};

    
   pub struct lexer {
        contents:Vec<char>,
        counter:usize,
    }
    impl lexer {
        pub fn new(c:String) -> Self {
           Self {
            contents:c.chars().collect(),
            counter:0,
           }
        }
        pub fn lex(&mut self) {
            let clength = self.contents.len();
            let mut tokens:Vec<Token> = Vec::<Token>::new();
            while self.counter < clength {   
                    match self.get_current_char() {
                      'f'  => {
                            let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut nextchars2:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    self.counter +=1;
                                    i += 1;
                                }
                            let s:String = nextchars.iter().collect();
                            if s == "func" {
                                tokens.push(Token::new(TokenType::Res_Function,"func".to_owned()));
                                self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for a function"); return; } 
                            }
                            else{continue;}
                      },  
                      '\"' => {
                        self.counter += 1;
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                        while self.get_current_char() != '\"' {
                            nextchars.push(self.get_current_char());
                            self.counter += 1;
                        }
                        tokens.push(Token::new(TokenType::STRING,nextchars.iter().collect()));
                      },
                      '\'' => {
                        self.counter += 1;
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                        while self.get_current_char() != '\'' {
                            nextchars.push(self.get_current_char());
                            self.counter += 1;
                        }
                        if nextchars.len() > 1 { println!("char can only hold 1 character"); return; }
                        else {
                        tokens.push(Token::new(TokenType::CHAR,nextchars.iter().collect()));
                        }
                      },

                      'S' => { 
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                        let mut nextchars2:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 6 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "String" {
                                tokens.push(Token::new(TokenType::Res_String,"String".to_owned()));
                                self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for a String"); return; } 

                            }
                            else{continue;}
                        },
                    'I' => { 
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                        let mut nextchars2:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 3 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "Int" {
                                tokens.push(Token::new(TokenType::Res_Int,"Int".to_owned()));
                                self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for an Interger"); return; } 

                            }
                            else{continue;}
                        },
                        'B' => {
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                        let mut nextchars2:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "Bool" {
                                tokens.push(Token::new(TokenType::Res_Bool,"Bool".to_owned()));
                                self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for a Boolean"); return; } 

                            }
                            else{continue;}
                        },
                    'C' => {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                        let mut nextchars2:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                  self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "Char" {
                                tokens.push(Token::new(TokenType::Res_Char,"Char".to_owned()));
                                self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for a char"); return; } 

                            }
                            else{continue;}
                    },
                    'U' => {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                    let mut nextchars2:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "UInt" {
                                tokens.push(Token::new(TokenType::Res_Uint,"UInt".to_owned()));
                                self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for a uint"); return; } 

                            }
                            else{continue;}
                            
                    },
                    'F' => {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                    let mut nextchars2:Vec<char> = Vec::<char>::new();
                        let mut i:usize = 0;    
                        while i < 5 {
                                nextchars.push(self.get_current_char());
                                i += 1;
                                self.counter +=1;
                            }
                        let s:String  = nextchars.iter().collect();
                        if s == "Float" {
                            tokens.push(Token::new(TokenType::Res_Float,"Float".to_owned()));
                            self.skip_white_space();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars2.push(self.get_current_char());
                                    self.counter +=1
                                }
                                if nextchars2 != [' '] || nextchars2 != ['\0'] {
                                tokens.push(Token::new(TokenType::Identifier,nextchars2.iter().collect()));
                                }
                               else { println!("somewhere your forgor to put identifier for a float"); return; } 

                        }
                        else{continue;}
                    },
                      '=' => {
                       tokens.push(Token::new(TokenType::Equal,"=".to_owned()));
                      },
                      '/' => {
                      if self.peek_next_token() == '/' {
                          while self.get_current_char() != '\n' {
                                self.counter +=1;
                      }
                        continue;
                    }
                      else {
                        tokens.push(Token::new(TokenType::Divide,"/".to_owned()));
                      }
                      },
                    '>' => {
                       tokens.push(Token::new(TokenType::Rarrow,">".to_owned()));
                      },
                     '<' => {
                       tokens.push(Token::new(TokenType::Larrow,"<".to_owned()));
                      },
                    '(' => {
                       tokens.push(Token::new(TokenType::LParen,"(".to_owned()));
                       let mut nextchars:Vec<char> = Vec::<char>::new();
                      // while self.get_current_char().is_alphabetic() {nextchars.push(self.get_current_char()); self.counter += 1;} // this is important for the parser
                       
                      },
                    ')' => {
                       tokens.push(Token::new(TokenType::Rparen,")".to_owned()));
                      },
                    '!' => {
                       if self.peek_next_token() == '!' {
                       tokens.push(Token::new(TokenType::Main,"Entrypoint".to_owned()));
                       self.counter += 1;
                       }
                       else {
                       tokens.push(Token::new(TokenType::Bang,"!".to_owned()));
                       }
                      },
                      '+' => {
                       tokens.push(Token::new(TokenType::Add,"+".to_owned()));
                      },
                      
                     '-' => {
                       tokens.push(Token::new(TokenType::Subtract,"-".to_owned()));
                      },  
                     '*' => {
                       tokens.push(Token::new(TokenType::Multiply,"*".to_owned()));
                      },
                    '&' => {
                        if self.peek_next_token() == '&'{
                         tokens.push(Token::new(TokenType::And,"&&".to_owned()));
                         self.counter += 2;
                        }
                        else {
                       tokens.push(Token::new(TokenType::And,"&".to_owned()));
                        }
                    }, 
                    '|' => {
                        if self.peek_next_token() == '|'{
                         tokens.push(Token::new(TokenType::Or,"||".to_owned()));
                         self.counter += 2;
                        }
                        else {
                       tokens.push(Token::new(TokenType::Or,"|".to_owned()));
                        }
                    },

                        _ => (), // we will let the parser do this
                    } 
                self.counter += 1;
                        }
            print!("{:?}",tokens);
            let mut file = File::create("lexed.frl")
                .expect("Couldnt create final lexed file");
            write!(file,"{:?}",tokens).expect("Couldnt write final lex");
                    }
                fn get_current_char(&self) -> char {
                     let cc = self.contents[self.counter]; return cc
                }
               fn skip_white_space(&mut self) {
                while self.get_current_char().is_whitespace() {
                    self.counter +=1;
                    if !self.get_current_char().is_whitespace() {
                        break;
                    }
                }
               }
               pub fn peek_next_token(&self) -> char {
                    let counter = &self.counter + 1;  
                    let cc:char = self.contents[counter.to_owned()]; return cc
                } 
               pub fn peek_next_couple_tokens(&self,amount:usize) -> char {
                    let counter = &self.counter + amount;  
                    let cc:char = self.contents[counter.to_owned()]; return cc
                }
               pub fn peek_back_couple_tokens(&self,amount:usize) -> char {
                    let counter = &self.counter - amount;  
                    let cc:char = self.contents[counter.to_owned()]; return cc
                }
 
            pub fn last_token(&self) -> char {
                    let counter = &self.counter - 1;  
                    let cc = self.contents[counter.to_owned()]; return cc
                }
    } 
