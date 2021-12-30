enum tokentype {
    Id, // foo
    Notation, // 1 + 1
    Int, // 123
    Literal, // "s" 
    Char, // 's'
    Function, // fn main(args here) { -_- code *-* } 
    Return, // 
}


struct token {
    
}


fn tokenize(file: String) {
   let wssfile = file.replace(" ","");
    for ct in wssfile.chars().next() {
        if ct == '+' {
            
        } 
    }

}

        

