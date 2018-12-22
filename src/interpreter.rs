use crate::model::{Expression, Literal};

type EvalResult = Result<Literal, String>;

pub fn evaluate(expr: Expression) -> EvalResult {
    match expr {
        Expression::Lit(literal) => Ok(literal),
        Expression::Neg(e) => Ok(evaluate(*e)?),
        Expression::Eq(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => Ok(Literal::Bool(a == b)),
                (Literal::String(a), Literal::String(b)) => Ok(Literal::Bool(a == b)),
                _ => Err(String::from("invalid types")),
            }
        }
        Expression::Add(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a + b)),
                (Literal::F64(a), Literal::F64(b)) => Ok(Literal::F64(a + b)),
                (Literal::String(a), Literal::String(b)) => {
                    Ok(Literal::String(a.chars().chain(b.chars()).collect()))
                }
                _ => Err(String::from("invalid types")),
            }
        }
        Expression::Sub(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a - b)),
                (Literal::F64(a), Literal::F64(b)) => Ok(Literal::F64(a - b)),
                _ => Err(String::from("invalid types")),
            }
        }
        Expression::Mul(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a * b)),
                (Literal::F64(a), Literal::F64(b)) => Ok(Literal::F64(a * b)),
                _ => Err(String::from("invalid types")),
            }
        }
        Expression::Div(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => {
                    if b != 0 {
                        Ok(Literal::I64(a / b))
                    } else {
                        Err(String::from("divide by zero"))
                    }
                }
                (Literal::F64(a), Literal::F64(b)) => Ok(Literal::F64(a / b)),
                _ => Err(String::from("invalid types")),
            }
        }
        Expression::Mod(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a % b)),
                _ => Err(String::from("invalid types")),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::evaluate;
    use crate::model::Literal;
    use crate::parsers::parse;

    #[test]
    fn smoke() {
        let input = r#" 1 + 2 + 3 + 4 + 5 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Ok(Literal::I64(1 + 2 + 3 + 4 + 5)),
        );
    }

    #[test]
    fn string_addition() {
        let input = r#" "asdf" + "pqrs" + "tuvw" == "asdfpqrstuvw" "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(true)),);
    }

    #[test]
    fn type_error_adding_string_and_int() {
        let input = r#" "asdf" + 5 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("invalid types"))
        );
    }

    #[test]
    fn type_error_subtracting_strings() {
        let input = r#" "asdf" - "pqrs" "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("invalid types"))
        );
    }

    #[test]
    fn eval_error_divide_by_zero_int() {
        let input = r#" 1 / 0 "#;
        assert_eq!(
            evaluate(parse(input).unwrap()),
            Err(String::from("divide by zero"))
        );
    }
}
