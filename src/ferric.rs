/// ferriclang's first line :)
use std::{env, process};
#[path = "./lexer/lexer.rs"]mod lexer;
#[path = "./lexer/token.rs"]mod tokens;
fn main() {
let args: Vec<String> = env::args().collect();
if args.len() == 1 {
    println!("ARGS not supplied");
    process::exit(1);
    }
    match args[1].as_str() {
    "-h" | "--help" => {
        print_help();
        process::exit(0);
        }
        "-v" | "--version" => {
        version();
        process::exit(0);
        }
    _ => {}
    }
}

fn version() {
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
println!("{}",
         format!(r"
your ferric version {}
                 ",VERSION.unwrap_or("unknow ver")))
}

fn print_help () {
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
println!("{}", 
        format!(
r"ferric {}
            
USAGE:
ferric [FILE] [ARGS]

FLAGS:
-h show this help screen!

ARGS:
compile and run a .ferric file! or just compile (WIP)
            ", VERSION.unwrap_or("unknown ver")
            ) 
        );

}

