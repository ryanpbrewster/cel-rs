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
    extract_relation(pair.into_inner().next().unwrap())
}

fn extract_relation(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Relation);
    let mut pairs = pair.into_inner();
    let a = extract_addition(pairs.next().unwrap());
    match pairs.next() {
        None => a,
        Some(op) => {
            assert_eq!(op.as_rule(), Rule::RelOp);
            let b = extract_addition(pairs.next().unwrap());
            match op.as_str() {
                "==" => Expression::Eq(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }
    }
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
        Rule::Relation => extract_relation(a),
        Rule::UnaryOp => {
            assert_eq!(a.as_rule(), Rule::UnaryOp);
            match a.as_str() {
                "-" => Expression::Neg(Box::new(extract_unary(pairs.next().unwrap()))),
                "!" => Expression::Not(Box::new(extract_unary(pairs.next().unwrap()))),
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
        Rule::StringLiteral => Literal::String(extract_string(pair)),
        Rule::IntLiteral => Literal::I64(pair.as_str().parse().unwrap()),
        Rule::ListLiteral => extract_list(pair),
        Rule::BoolLiteral => Literal::Bool(pair.as_str().parse().unwrap()),
        _ => unreachable!(),
    }
}

fn extract_string(pair: Pair<Rule>) -> String {
    assert_eq!(pair.as_rule(), Rule::StringLiteral);
    let mut buf = String::new();
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::CharLiteral => {
                buf.push_str(p.as_str());
            }
            Rule::Escape => {
                let s = &p.as_str()[1..];
                match s {
                    "t" => buf.push('\t'),
                    "n" => buf.push('\n'),
                    "\"" => buf.push('"'),
                    _ => {
                        buf.push(u8::from_str_radix(s, 8).unwrap() as char);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    buf
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

    fn assert_invalid(input: &str) {
        let parsed = parse(input);
        assert!(parsed.is_err(), "{} was accepted as {:?}", input, parsed);
    }

    #[test]
    fn cel_valid() {
        assert_valid("22 * (4 + 15)");
        assert_valid("22 * -4");
        assert_valid("!false");
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
        let input = "[0, '1', 2 + '3']";
        let res = assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::List(vec![
                Expression::Lit(Literal::I64(0)),
                Expression::Lit(Literal::String(String::from("1"))),
                Expression::Add(
                    Box::new(Expression::Lit(Literal::I64(2))),
                    Box::new(Expression::Lit(Literal::String(String::from("3")))),
                )
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
    fn cel_string_octal_escapes() {
        assert_valid(r#" "\0" "#);
        assert_valid(r#" "\7" "#);
        assert_valid(r#" "\07" "#);
        assert_valid(r#" "\77" "#);
        assert_valid(r#" "\377" "#);
        assert_invalid(r#" "\8" "#);
        assert_valid(r#" "\400" "#); // parsed as [\4, 0, 0]
    }
}
