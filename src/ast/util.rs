use crate::token::TokenType;
pub struct Astnode {
    value:String,     
    children:Vec<Astnode>,
}
pub struct Ast {
    tree:Astnode
}
fn getChildren<F: FnMut(&String)> (node:&Astnode,f: &mut F) {
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
       getChildren(node,f);
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
    getChildren(&self.tree,&mut |x| println!("{}",x)); 
    }
}
#[allow(unused_variables)] // here right now to stop warnings
pub fn astgen (lexed:Vec<TokenType>){
// get mainnode
let mainnode:Astnode;
let ast:Ast;
let mut iter = lexed.iter();
let maintoken = iter.position(|x| x == &TokenType::Main).unwrap(); // find the first Main token 
let mainident:String;
    // work around so I can get the string I want
    match &lexed[maintoken - 1 as usize].to_owned() {
        TokenType::Identifier(i) => mainident = i.to_owned(), 
        _ => ()
    }
// functions should have 2 children, return:type and contents:Vec<Node>
// parse everything after into children

// TODO parse everything before into imports(not really but I dont know the word you would use to
// call it maybe inserts?,just something that it can call or use)
}
