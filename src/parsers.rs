use crate::model::{Expression, Literal};

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cel.pest"]
struct CelParser;

pub fn parse(input: &str) -> Result<Expression, String> {
    let mut parsed =
        CelParser::parse(Rule::Expression, input).map_err(|err| format!("{:?}", err))?;
    Ok(extract_expression(parsed.next().unwrap()))
}

fn extract_expression(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Expression);
    extract_addition(pair.into_inner().next().unwrap())
}

fn extract_addition(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Addition);
    let mut pairs = pair.into_inner();
    let a = extract_multiplication(pairs.next().unwrap());
    match pairs.next() {
        None => a,
        Some(op) => {
            assert_eq!(op.as_rule(), Rule::AddOp);
            let b = extract_addition(pairs.next().unwrap());
            match op.as_str() {
                "+" => Expression::Add(Box::new(a), Box::new(b)),
                "-" => Expression::Sub(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }
    }
}

fn extract_multiplication(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Multiplication);
    let mut pairs = pair.into_inner();
    let a = extract_unary(pairs.next().unwrap());
    match pairs.next() {
        None => a,
        Some(op) => {
            assert_eq!(op.as_rule(), Rule::MulOp);
            let b = extract_multiplication(pairs.next().unwrap());
            match op.as_str() {
                "*" => Expression::Mul(Box::new(a), Box::new(b)),
                "/" => Expression::Div(Box::new(a), Box::new(b)),
                "%" => Expression::Mod(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }
    }
}

fn extract_unary(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Unary);
    let mut pairs = pair.into_inner();
    let a = pairs.next().unwrap();
    match a.as_rule() {
        Rule::Literal => Expression::Lit(extract_literal(a)),
        Rule::Addition => extract_addition(a),
        Rule::UnaryOp => {
            assert_eq!(a.as_rule(), Rule::UnaryOp);
            match a.as_str() {
                "-" => Expression::Neg(Box::new(extract_unary(pairs.next().unwrap()))),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn extract_literal(pair: Pair<Rule>) -> Literal {
    assert_eq!(pair.as_rule(), Rule::Literal);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::StringLiteral => {
            let s = pair.as_str();
            Literal::String(String::from(&s[1..s.len() - 1]))
        }
        Rule::IntLiteral => Literal::I64(pair.as_str().parse().unwrap()),
        Rule::ListLiteral => extract_list(pair),
        _ => unreachable!(),
    }
}

fn extract_list(pair: Pair<Rule>) -> Literal {
    assert_eq!(pair.as_rule(), Rule::ListLiteral);
    let mut vs = Vec::new();
    for p in pair.into_inner() {
        vs.push(extract_addition(p));
    }
    Literal::List(vs)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Literal;

    fn assert_valid(input: &str) {
        parse(input).expect("failed to parse");
    }

    #[test]
    fn cel_valid() {
        assert_valid("22 * (4 + 15)");
        assert_valid("22 * -4");
    }

    #[test]
    fn cel_smoke() {
        let input = "22 * (4 + 15)";
        let res = assert_eq!(
            parse(input),
            Ok(Expression::Mul(
                Box::new(Expression::Lit(Literal::I64(22))),
                Box::new(Expression::Add(
                    Box::new(Expression::Lit(Literal::I64(4))),
                    Box::new(Expression::Lit(Literal::I64(15))),
                ))
            ))
        );
    }

    #[test]
    fn cel_list() {
        let input = "[0, 1, 2]";
        let res = assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::List(vec![
                Expression::Lit(Literal::I64(0)),
                Expression::Lit(Literal::I64(1)),
                Expression::Lit(Literal::I64(2)),
            ])))
        );
    }

    #[test]
    fn cel_string() {
        let input = r#""asdf""#;
        let res = assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::String(String::from("asdf"))))
        );
    }

    #[test]
    fn cel_escaped_quote_string() {
        let input = r#""as\"df""#;
        let res = assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::String(String::from("as\"df"))))
        );
    }

    #[test]
    fn cel_bad_string() {
        let input = r#""\0""#;
        assert_eq!(parse(input), Err(String::from("bad literal")),);
    }
}
