use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Eq(Box<Expression>, Box<Expression>),
    Neq(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Lte(Box<Expression>, Box<Expression>),
    Gte(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Not(Box<Expression>),
    Method(Box<Expression>, MethodName, Vec<Expression>),
    Lit(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Expression>),
    Null,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MethodName {
    Len,
    Pow,
}

impl FromStr for MethodName {
    type Err = String;
    fn from_str(s: &str) -> Result<MethodName, String> {
        match s {
            "len" => Ok(MethodName::Len),
            "pow" => Ok(MethodName::Pow),
            _ => Err(format!("unknown method '{}'", s)),
        }
    }
}
