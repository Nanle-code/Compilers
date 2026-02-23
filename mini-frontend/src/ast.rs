#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOpKind,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOpKind {
   Add,
   Mul,
}

#[derive(Debug)]
pub enum Statement{
    Let {
        name: String,
        value: Expr,
    },
    Expr(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}