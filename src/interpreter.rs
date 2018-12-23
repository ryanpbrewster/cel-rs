use crate::model::{Expression, Literal, MethodName};

type EvalResult = Result<Literal, String>;

pub fn evaluate(expr: Expression) -> EvalResult {
    match expr {
        Expression::Lit(literal) => Ok(literal),
        Expression::Neg(e) => match evaluate(*e)? {
            Literal::I64(x) => Ok(Literal::I64(-x)),
            _ => Err(String::from("invalid types")),
        },
        Expression::Not(e) => match evaluate(*e)? {
            Literal::Bool(x) => Ok(Literal::Bool(!x)),
            _ => Err(String::from("invalid types")),
        },
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
                _ => Err(String::from("invalid types")),
            }
        }
        Expression::Mul(a, b) => {
            let a = evaluate(*a)?;
            let b = evaluate(*b)?;
            match (a, b) {
                (Literal::I64(a), Literal::I64(b)) => Ok(Literal::I64(a * b)),
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
        Expression::Method(e, name, args) => {
            let e = evaluate(*e)?;
            match e {
                Literal::String(a) => match name {
                    MethodName::Len => {
                        if !args.is_empty() {
                            return Err(String::from("too may arguments to .len()"));
                        }
                        Ok(Literal::I64(a.chars().count() as i64))
                    }
                    MethodName::Pow => Err(String::from("illegal type for .pow()")),
                },
                Literal::I64(a) => match name {
                    MethodName::Len => Err(String::from("illegal type for .len()")),
                    MethodName::Pow => {
                        if args.len() != 1 {
                            return Err(String::from("too may arguments to .pow()"));
                        }
                        match evaluate(args[0].clone())? {
                            Literal::I64(b) => Ok(Literal::I64(i64::pow(a, b as u32))),
                            _ => Err(String::from("illegal type for .pow()")),
                        }
                    }
                },
                other => Err(format!("illegal method call {:?}.{:?}", other, name)),
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
    fn unary_negative() {
        let input = r#" -5 + 8 "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(3)),);
    }

    #[test]
    fn unary_not() {
        let input = r#" !(5 + 5 == 10) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(false)),);
    }

    #[test]
    fn string_addition() {
        let input = r#" "asdf" + "pqrs" + "tuvw" == "asdfpqrstuvw" "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::Bool(true)),);
    }

    #[test]
    fn string_len() {
        let input = r#" "asdf".len() + "pqrs".len() "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(8)),);
    }

    #[test]
    fn int_pow() {
        let input = r#" 42.pow(2) "#;
        assert_eq!(evaluate(parse(input).unwrap()), Ok(Literal::I64(42 * 42)),);
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
