#[derive(Debug, PartialEq)]
pub enum Expression {
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
    Lit(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    I64(i64),
    F64(f64),
}
