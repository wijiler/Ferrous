use crate::token::TokenType;
pub enum NodeType {
    BinaryOp{a:TokenType,b:TokenType},
    Reserved,
    Declaration{Type:TokenType,ident:String,equal:Box<NodeType>},
    Args,
    Entrypoint,
    Function{entry:bool,ident:String,args:Option<Vec<AstNode>>,contents:Vec<AstNode>},
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
