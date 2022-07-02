extern crate pest;
use pest::Parser;
use crate::lexer::Lexer;
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Ferparser;

fn parse (file:String) {
let mut lexer = Lexer::new(file);
let tokens = lexer.lex();
//let pairs = Ferparser::parse(Rule::program,format!("{:?}",&tokens)).unwrap_or_else(|e| panic!("{}",e));

}
