lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
lalrpop_mod!(pub calculator3); // synthesized by LALRPOP
lalrpop_mod!(pub cel); // synthesized by LALRPOP

#[cfg(test)]
mod test {
    use crate::model::{Expression, Literal};

    #[test]
    fn smoke1() {
        let input = "22";
        let res = assert_eq!(super::calculator1::TermParser::new().parse(input), Ok(22));
    }

    #[test]
    fn smoke3() {
        let input = "22 * (4 + 15)";
        let res = assert_eq!(
            super::calculator3::ExprParser::new().parse(input),
            Ok(22 * (4 + 15))
        );
    }

    #[test]
    fn smoke_cel() {
        let input = "22 * (4 + 15)";
        let res = assert_eq!(
            super::cel::ExprParser::new().parse(input),
            Ok(Expression::Mul(vec![
                Expression::Lit(Literal::I64(22)),
                Expression::Add(vec![
                    Expression::Lit(Literal::I64(4)),
                    Expression::Lit(Literal::I64(15)),
                ])
            ]))
        );
    }
}
