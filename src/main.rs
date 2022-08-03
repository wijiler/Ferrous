/// ferriclang's first line :)
use std::{env, fs, process};
#[macro_use]
pub mod lexer;
pub mod parser;
pub mod token;
pub mod ast; // cant call them both parser
// args handler so you can use the thing
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
        "--comp" | "-c" => {
            let filename = &args[2];
            let file = fs::read_to_string(filename).unwrap();
            //        file.retain(|c| !c.is_whitespace());
            let mut lexer = lexer::Lexer::new(file);
            lexer.lex();
        }
        _ => {}
    }
}
// prints the version
fn version() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        format!(
            r"
your ferric version {}
                 ",
            VERSION.unwrap_or("unknow ver")
        )
    )
}
// help cuz help
fn print_help() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        format!(
            r"ferric {}
            
USAGE:
ferric [ARGS] [FILE]
FLAGS:
-h show this help screen!
-v show version
-c compile
What Does this tool do?:
allows you to compile .fr or .ferric files
            ",
            VERSION.unwrap_or("unknown ver")
        )
    );
}
