use super::*;

use nom::{branch::alt, bytes::complete::tag, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    None,
    Ellipsis,
    Pass,
}

pub fn expr(input: &str) -> ParseResult<Expr> {
    alt((
        tag("None").map(|_| Expr::None),
        tag("...").map(|_| Expr::Ellipsis),
        tag("pass").map(|_| Expr::Pass),
    ))
    .parse(input)
}
