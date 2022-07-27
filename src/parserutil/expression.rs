#[derive(Debug,Clone)]
pub enum Operations{
    Plus,
    Minus,
    Divide,
    Multiply,
}
/// credits to Eskpill for this argument and call code. https://github.com/Eskpil 
#[derive(Debug,Clone)]
pub struct Argument {
    value:Expr,
    name:String,
}
#[derive(Debug,Clone)]
pub enum Expr {
    Int(i64),
    UnaryExpr {
        op:Operations,
        child:Box<Expr>,
    },
    BinaryExpr {
        op:Operations,
        lhs:Box<Expr>,
        rhs:Box<Expr>
    },
    StrLiteral(String),
    Ident(String),
    Bool(bool),
    Literal(i64),
    Call{func:String,args:Vec<Argument>},
    NLookup{ name:String,value:String }
}
