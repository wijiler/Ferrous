use token::TokenType;
#[derive(Debug,Clone)]
pub struct Parser {
   tokens:Vec<TokenType>, 
}

impl Parser {
    pub fn new (t:Vec<TokenType>) -> Self {
        Self {
            tokens:t
        }
    }
    pub fn peek(&self) -> TokenType {
        self.tokens.to_vec().into_iter().next().unwrap()
    }  
    pub fn nom (self) -> TokenType { // its the eat/consume function but I think its funnier to call it nom
        self.tokens.to_vec().into_iter().next().unwrap()
    }
}
pub struct Astnode {
    value:String,     
    children:Vec<Astnode>,
}
pub struct Ast {
    tree:Astnode
}
fn getchildren<F: FnMut(&String)> (node:&Astnode,f: &mut F) {
   let mut children:Vec<String> = Vec::new();
   let closure = |x| x;
   if node.children.len() >= 1 {
       for child in &node.children {
       let s = closure(&child.value);
       children.push(s.to_string());
       }
       f(&(node.value.to_string() + " -> " + &children.join(",")));
   }
   for node in &node.children {
       getchildren(node,f)
   }
}
impl Astnode {
    pub fn new (v:String,c:Vec<Astnode>) -> Self {
       Self {
           value:v,
           children:c,
       }
    }
}
impl Ast {
    pub fn new(t:Astnode) -> Self {
        Self {
            tree:t
        }
    }
    pub fn print(&self) {
    getchildren(&self.tree,&mut |x| println!("{}",x)); 
    }
}
