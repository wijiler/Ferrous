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
            let clength = self.contents.len();
            let mut tokens:Vec<Token> = Vec::<Token>::new();
            while self.counter < clength {   
                    match self.get_current_char() {
                      'f' => {
                            let mut nextchars:Vec<char> = Vec::<char>::new();
                                while self.get_current_char().is_alphabetic() {
                                    nextchars.push(self.get_current_char());
                                    self.counter +=1;
                                }
                            if nextchars == ['f','u','n','c'] {
                                tokens.push(Token::new(TokenType::Res_Function,"func".to_owned()));
                            }
                            else  {
                                while self.get_current_char().is_alphabetic() {
                                    nextchars.push(self.get_current_char())
                                }
                                tokens.push(Token::new(TokenType::Identifier,nextchars.into_iter().collect()));
                            }
                      },  
                      '\"' => {
                          self.counter += 1;
                        let mut nextchars:Vec<char> = Vec::<char>::new();
                        while self.get_current_char() != '\"' {
                            nextchars.push(self.get_current_char());
                            self.counter += 1;
                        }
                        tokens.push(Token::new(TokenType::STRING,nextchars.into_iter().collect()));
                      },
                      '=' => {
                       tokens.push(Token::new(TokenType::Equal,"=".to_owned()));
                      }
                        _ => (), // TODO:Identifiers
                    } 
                self.counter += 1;
            }
            print!("{:?}",tokens);
        }
                fn get_current_char(&self) -> char {
                     let cc = self.contents[self.counter]; return cc
                }
               /* 
               pub fn peek_next_token(&mut self) -> char {
                    self.counter += 1; 
                    let next_token = self.get_current_char();
                    return next_token
                }
                */
    }
