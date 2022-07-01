extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Ferparser;

