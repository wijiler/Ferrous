use std::fs;
pub fn tokenize(filename:&String) {
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading please try again");
    // check if file is a ferriclang file
    let mut extension: String = filename.chars().rev().take(2).collect();
    extension = extension.pop().unwrap().to_string() + &extension;
    if extension != "fr" {
       panic!("not a .fr file please try again"); 
    }
    contents.retain(|c| !c.is_whitespace());
   println!("File contents:\n{}",contents);
}
