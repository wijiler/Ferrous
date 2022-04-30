use crate::token::*;


    
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
            self.peek_next_token();
            let clength = self.contents.len();
            let mut tokens:Vec<Token> = Vec::<Token>::new();
            while self.counter < clength {   
                    match self.get_current_char() {
                      'f'  => {
                            let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    self.counter +=1;
                                    i += 1;
                                }
                            let s:String = nextchars.iter().collect();
                            if s == "func" {
                                tokens.push(Token::new(TokenType::Res_Function,"func".to_owned()));
                            }
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
                      'S' => { 
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 6 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "String" {
                                tokens.push(Token::new(TokenType::Res_String,"String".to_owned()));
                            }
                        },
                    'I' => { 
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 3 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "Int" {
                                tokens.push(Token::new(TokenType::Res_Int,"Int".to_owned()));
                            }
                        },
                        'B' => {
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "Bool" {
                                tokens.push(Token::new(TokenType::Res_Bool,"Bool".to_owned()));
                            }
                        },
                    'C' => {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                   self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "Char" {
                                tokens.push(Token::new(TokenType::Res_Char,"Char".to_owned()));
                            }
                    },
                    'U' => {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                            let mut i:usize = 0;    
                            while i < 4 {
                                    nextchars.push(self.get_current_char());
                                    i += 1;
                                    self.counter +=1;
                                }
                            let s:String  = nextchars.iter().collect();
                            if s == "UInt" {
                                tokens.push(Token::new(TokenType::Res_Uint,"UInt".to_owned()));
                            }
                            
                    },
                    'F' => {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                        let mut i:usize = 0;    
                        while i < 5 {
                                nextchars.push(self.get_current_char());
                                i += 1;
                                self.counter +=1;
                            }
                        let s:String  = nextchars.iter().collect();
                        if s == "Float" {
                            tokens.push(Token::new(TokenType::Res_Float,"Float".to_owned()));
                        }
                    },
                      '=' => {
                       tokens.push(Token::new(TokenType::Equal,"=".to_owned()));
                      },
                      '/' => {
                      if self.peek_next_token() == '/' {self.counter += 2;continue;}
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
                      },
                    ')' => {
                       tokens.push(Token::new(TokenType::Rparen,")".to_owned()));
                      },
                    '!' => {
                       if self.peek_next_token() == '!' {
                       tokens.push(Token::new(TokenType::Main,"Entrypoint".to_owned())); 
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

                        _ => {
                        if self.get_current_char() != ' '{
                                if self.record_tokens(8,6) == "String" || self.record_tokens(6,4) == "func" {}
                            }
                        }, 
                    } 
                self.counter += 1;
                        }
            print!("{:?}",tokens);
                    }
                fn get_current_char(&self) -> char {
                     let cc = self.contents[self.counter]; return cc
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
  pub fn record_tokens(&mut self,amountback:usize,amountforward:usize) -> String {
                    let mut nextchars:Vec<char> = Vec::<char>::new();
                    let mut i = 0;
                    self.counter -= amountback;
                    while i < amountforward {
                        nextchars.push(self.get_current_char());
                            i +=1;
                            self.counter += 1;
                    }
                    return nextchars.into_iter().collect();
                }
 
            pub fn last_token(&self) -> char {
                    let counter = &self.counter - 1;  
                    let cc = self.contents[counter.to_owned()]; return cc
                }
    } 
