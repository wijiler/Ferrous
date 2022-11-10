use crate::token::TokenType;
pub struct Astnode {
    value:String,     
    children:Vec<Astnode>,
}
pub struct Ast {
    tree:Astnode
}
fn printable<F: FnMut(&String)> (node:&Astnode,f: &mut F) {
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
       printable(node,f)
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
    printable(&self.tree,&mut |x| println!("{}",x)); 
    }
}
#[allow(unused_variables)] // here right now to stop warnings
pub fn astgen (lexed:Vec<TokenType>) {
    let mainnode:Astnode;
    let ast:Ast;
    let mut iter = lexed.into_iter();
    let maintoken = iter.position(|x| x == TokenType::Main).to_owned(); // find the first Main token 
}
