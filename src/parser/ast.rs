#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    BinOp(Box<Expr>, OpCode, Box<Expr>),
}