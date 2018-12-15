lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
lalrpop_mod!(pub calculator3); // synthesized by LALRPOP

#[cfg(test)]
mod test {
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
}
