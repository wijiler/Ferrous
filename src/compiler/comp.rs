// compiler pog

use std::io::{Result, stdin};
use crate::lexer::token::*;

// just take in the filename
pub fn find_file() -> Result<()> {
    let mut filename = String::new();
    println!("File To Compile:"); 
    let _filetocompile = stdin().read_line(&mut filename)?;
    filename.pop();
    Ok(())
}

pub fn comp() -> Result<()> {
    find_file()?;

    Ok(())
}
