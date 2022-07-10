pub struct Function {
    
}
pub enum NodeType {
    Constant(f64),
    Literal(String), 

}
pub struct AstNode {
    nodetype:NodeType,
    children:Option<Vec<AstNode>>
}
impl AstNode {
    pub fn new (nt:NodeType,c:Option<Vec<AstNode>>) -> Self {
        Self {
            nodetype:nt,
            children:c,
        }
    }
}
