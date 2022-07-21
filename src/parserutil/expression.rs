// can I get some of dat bt-7274-ussy
pub enum Operations{
    Plus,
    Minus,
    Divide,
    Multiply,
}
 
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
    StrL(String),
    Ident(String),

}
