/// ferriclang's first line :) penis
use std::{env, process};

mod compiler {
   pub mod comp;
   pub use self::comp::comp;
}
mod lexer {
   pub mod token;
    pub use self::token::*;
}

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
        "--test" | "-t" => {
            compiler::comp::comp();
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

