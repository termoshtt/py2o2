use super::*;

use nom::{branch::alt, bytes::complete::tag, sequence::tuple, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'input> {
    Name { id: Identifier<'input> },
    None,
    Ellipsis,
    Pass,
}

pub fn expr(input: &str) -> ParseResult<Expr> {
    alt((
        tag("None").map(|_| Expr::None),
        tag("...").map(|_| Expr::Ellipsis),
        tag("pass").map(|_| Expr::Pass),
        identifier.map(|id| Expr::Name { id }),
    ))
    .parse(input)
}

/// Comparison operator
///
/// ```text
/// cmpop = Eq | NotEq | Lt | LtE | Gt | GtE | Is | IsNot | In | NotIn
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

pub fn cmpop(input: &str) -> ParseResult<CmpOp> {
    alt((
        tuple((tag("is"), multispace1, tag("not"))).map(|_| CmpOp::IsNot),
        tag("is").map(|_| CmpOp::Is),
        tuple((tag("not"), multispace1, tag("in"))).map(|_| CmpOp::NotIn),
        tag("in").map(|_| CmpOp::In),
        tag("<=").map(|_| CmpOp::LtE),
        tag("<").map(|_| CmpOp::Lt),
        tag(">=").map(|_| CmpOp::GtE),
        tag(">").map(|_| CmpOp::Gt),
        tag("!=").map(|_| CmpOp::NotEq),
        tag("==").map(|_| CmpOp::Eq),
    ))
    .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;

    #[test]
    fn test_cmpop() {
        assert_eq!(cmpop(">").finish().unwrap(), ("", CmpOp::Gt));
        assert_eq!(cmpop(">=").finish().unwrap(), ("", CmpOp::GtE));
        assert_eq!(cmpop("<").finish().unwrap(), ("", CmpOp::Lt));
        assert_eq!(cmpop("<=").finish().unwrap(), ("", CmpOp::LtE));
        assert_eq!(cmpop("==").finish().unwrap(), ("", CmpOp::Eq));
        assert_eq!(cmpop("!=").finish().unwrap(), ("", CmpOp::NotEq));
        assert_eq!(cmpop("is").finish().unwrap(), ("", CmpOp::Is));
        assert_eq!(cmpop("is not").finish().unwrap(), ("", CmpOp::IsNot));
        assert_eq!(cmpop("in").finish().unwrap(), ("", CmpOp::In));
        assert_eq!(cmpop("not in").finish().unwrap(), ("", CmpOp::NotIn));
    }
}
