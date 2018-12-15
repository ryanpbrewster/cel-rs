use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
    Lit(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Expression>),
    Map(HashMap<MapKey, Expression>),
    Null,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MapKey {
    I64(i64),
    U64(u64),
    Bool(bool),
    String(String),
}
