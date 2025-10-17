#[derive(Debug)]
pub struct Ident {
    value: String,
}

#[derive(Debug)]
pub struct NumberExpr {
    value: usize,
    raw: String,
}

#[derive(Debug)]
pub struct StringExpr {
    value: String,
}

#[derive(Debug)]
pub enum Expression {
    Number(NumberExpr),
    String(StringExpr),
}

#[derive(Debug)]
pub struct LetStatement {
    name: Ident,
    expr: Expression,
}

pub enum Statement {
    Let(LetStatement),
}
