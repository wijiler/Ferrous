pub enum NodeType {
    BinaryOp,
    Reserved,
    Declaration,
    Args,
    Entrypoint,
    Function{Identifier:String,args:Option<Vec<AstNode>>,contents:Vec<AstNode>},
    Str{value:String},
    Return{code:bool},
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
