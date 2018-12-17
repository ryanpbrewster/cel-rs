use crate::model::Expression;

lalrpop_mod!(pub cel); // synthesized by LALRPOP

pub fn parse(input: &str) -> Result<Expression, String> {
    cel::ExprParser::new()
        .parse(input)
        .map_err(|err| format!("{:?}", err))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Literal;

    #[test]
    fn cel_smoke() {
        let input = "22 * (4 + 15)";
        let res = assert_eq!(
            cel::ExprParser::new().parse(input),
            Ok(Expression::Mul(vec![
                Expression::Lit(Literal::I64(22)),
                Expression::Add(vec![
                    Expression::Lit(Literal::I64(4)),
                    Expression::Lit(Literal::I64(15)),
                ])
            ]))
        );
    }

    #[test]
    fn cel_list() {
        let input = "[0, 1, 2]";
        let res = assert_eq!(
            cel::ExprParser::new().parse(input),
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
            cel::ExprParser::new().parse(input),
            Ok(Expression::Lit(Literal::String(String::from("asdf"))))
        );
    }

    #[test]
    fn cel_escaped_quote_string() {
        let input = r#""as\"df""#;
        let res = assert_eq!(
            cel::ExprParser::new().parse(input),
            Ok(Expression::Lit(Literal::String(String::from("as\"df"))))
        );
    }

    #[test]
    fn cel_bad_string() {
        let input = r#""\0""#;
        assert_eq!(parse(input), Err(String::from("bad literal")),);
    }
}
