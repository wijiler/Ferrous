// compiler pog

use crate::lexer::token::*;
use std::io::{stdin, Result};

// just take in the filename so it knows what to compile
pub fn find_file() -> Result<()> {
    let mut filename = String::new();
    println!("File To Compile:");
    let _filetocompile = stdin().read_line(&mut filename)?;
    filename.to_string();
    filename.pop();

    let mut finalfilename = filename + ".ferric";
    finalfilename.pop();
    Ok(())
}
// advance the lexer so it can actually read the file
pub fn lexadvancer() {}

// run all the functions
pub fn comp() -> Result<()> {
    find_file()?;

    Ok(())
}
