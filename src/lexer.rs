use crate::token::*;
//use std::{fs::File, io::Write};
/// token system by me lexer is credited to https://www.youtube.com/channel/UCK6-JHDgbdeXbnTwc2Lj98g
pub struct Lexer {
    contents: Vec<char>,
    counter: usize,
}
impl Lexer {
    pub fn new(c: String) -> Self {
        Self {
            contents: c.chars().collect(),
            counter: 0,
        }
    }
    pub fn lex(&mut self) -> Vec<TokenType> {
        let mut nextnum: Vec<char> = Vec::new();
        let clength = self.contents.len();
        let mut tokens: Vec<TokenType> = Vec::new();
        while self.counter < clength {
            match self.get_current_char() {
                'f' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 4 {
                        nextchars.push(self.get_current_char());
                        self.counter += 1;
                        i += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "func" {
                        tokens.push(TokenType::Res_Function);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for a function");
                        }
                    } else {
                        continue;
                    }
                }
                '\"' => {
                    self.counter += 1;
                    let mut nextchars: Vec<char> = Vec::new();
                    while self.get_current_char() != '\"' {
                        nextchars.push(self.get_current_char());
                        self.counter += 1;
                    }
                    tokens.push(TokenType::StringL {
                        value: nextchars.iter().collect(),
                    });
                }
                '\'' => {
                    self.counter += 1;
                    let mut nextchars: Vec<char> = Vec::new();
                    while self.get_current_char() != '\'' {
                        nextchars.push(self.get_current_char());
                        self.counter += 1;
                    }
                    if nextchars.len() > 1 {
                        panic!("char can only hold 1 character");
                    } else {
                        tokens.push(TokenType::CharL {
                            value: nextchars.iter().collect(),
                        });
                    }
                }

                'S' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 6 {
                        nextchars.push(self.get_current_char());
                        i += 1;
                        self.counter += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "String" {
                        tokens.push(TokenType::Res_String);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for a String");
                        }
                    } else {
                        continue;
                    }
                }
                'I' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 3 {
                        nextchars.push(self.get_current_char());
                        i += 1;
                        self.counter += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "Int" {
                        tokens.push(TokenType::Res_Int);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for an Interger");
                        }
                    } else {
                        continue;
                    }
                }
                'B' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 4 {
                        nextchars.push(self.get_current_char());
                        i += 1;
                        self.counter += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "Bool" {
                        tokens.push(TokenType::Res_Bool);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for a Boolean");
                        }
                    } else {
                        continue;
                    }
                }
                'c' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 4 {
                        nextchars.push(self.get_current_char());
                        i += 1;
                        self.counter += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "char" {
                        tokens.push(TokenType::Res_Char);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for a char");
                        }
                    } else {
                        continue;
                    }
                }
                'u' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 4 {
                        nextchars.push(self.get_current_char());
                        i += 1;
                        self.counter += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "uInt" {
                        tokens.push(TokenType::Res_Uint);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for a uint");
                        }
                    } else {
                        continue;
                    }
                }
                'F' => {
                    let mut nextchars: Vec<char> = Vec::new();
                    let mut nextchars2: Vec<char> = Vec::new();
                    let mut i: usize = 0;
                    while i < 5 {
                        nextchars.push(self.get_current_char());
                        i += 1;
                        self.counter += 1;
                    }
                    let s: String = nextchars.iter().collect();
                    if s == "Float" {
                        tokens.push(TokenType::Res_Float);
                        self.skip_white_space();
                        while self.get_current_char().is_alphabetic() {
                            nextchars2.push(self.get_current_char());
                            self.counter += 1
                        }
                        if nextchars2 != [' '] || nextchars2 != ['\0'] {
                            tokens.push(TokenType::Identifier {
                                name: nextchars2.iter().collect(),
                            });
                        } else {
                            panic!("somewhere your forgor to put identifier for a float");
                        }
                    } else {
                        continue;
                    }
                }
                '=' => {
                    tokens.push(TokenType::Equal);
                }
                '/' => {
                    if self.peek_next_token() == '/' {
                        while self.get_current_char() != '\n' {
                            self.counter += 1;
                        }
                        continue;
                    } else {
                        tokens.push(TokenType::Divide);
                    }
                }
                ':' => {
                    tokens.push(TokenType::Colon)
                }
                ',' => {
                    tokens.push(TokenType::Comma);
                }
                '>' => {
                    tokens.push(TokenType::Rarrow);
                }
                '<' => {
                    tokens.push(TokenType::Larrow);
                }
                '(' => {
                    tokens.push(TokenType::LParen);
                }
                ')' => {
                    tokens.push(TokenType::Rparen);
                }
                '!' => {
                    if self.peek_next_token() == '!' {
                        tokens.push(TokenType::Main);
                        self.counter += 1;
                    } else {
                        tokens.push(TokenType::Bang);
                    }
                }
                '+' => {
                    tokens.push(TokenType::Add);
                }

                '-' => {
                    tokens.push(TokenType::Subtract);
                }
                '*' => {
                    tokens.push(TokenType::Multiply);
                }
                '&' => {
                    if self.peek_next_token() == '&' {
                        tokens.push(TokenType::And);
                        self.counter += 2;
                    } else {
                        tokens.push(TokenType::And);
                    }
                }
                '|' => {
                    if self.peek_next_token() == '|' {
                        tokens.push(TokenType::Or);
                        self.counter += 2;
                    } else {
                        tokens.push(TokenType::Or);
                    }
                }
                '0'..='9' => {
                    nextnum.push(self.get_current_char());
                    //self.counter +=1;
                    while self.peek_next_token() != ';' {
                        self.counter += 1;
                        nextnum.push(self.get_current_char());
                    }
                    let s: String = nextnum.iter().collect();
                    if !s.contains('.') {
                        tokens.push(TokenType::IntNumber { value: s });
                        nextnum.clear();
                    } else if s.matches('.').count() > 1 {
                        panic!("type float cannot have more than one decimal point");
                    } else {
                        tokens.push(TokenType::FloatNumber { value: s });
                        nextnum.clear();
                    }
                }
                '.' => {
                    nextnum.push(self.get_current_char());
                }
                'm' => {
                    if self.peek_next_token() == 'u' {
                        self.counter += 1;
                        if self.peek_next_token() == 't' {
                            self.counter += 1;
                            if !self.peek_next_token().is_whitespace() {
                                self.counter += 1;
                            } else {
                                tokens.push(TokenType::Mutable);
                                self.counter += 1;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                '{' => {
                    tokens.push(TokenType::Lbrack);
                }
                '}' => {
                    tokens.push(TokenType::Rbrack);
                }
                _ => (),
            }
            self.counter += 1;
        }
        print!("{:?}", tokens);
        tokens
    }
    fn get_current_char(&self) -> char {
        let cc = self.contents[self.counter];
        return cc;
    }
    fn skip_white_space(&mut self) {
        while self.get_current_char().is_whitespace() {
            self.counter += 1;
            if !self.get_current_char().is_whitespace() {
                break;
            }
        }
    }
    pub fn peek_next_token(&self) -> char {
        let counter = &self.counter + 1;
        let cc: char = self.contents[counter];
        return cc;
    }
}
