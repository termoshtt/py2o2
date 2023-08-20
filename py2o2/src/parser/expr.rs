use super::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::opt,
    multi::separated_list0,
    number::complete::double,
    sequence::tuple,
    Parser,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr<'input> {
    /// `Name(identifier id, expr_context ctx)`
    Name {
        id: &'input str,
    },
    /// `Constant(constant value, string? kind)`
    Constant {
        value: Constant<'input>,
    },
    /// `Tuple(expr* elts, expr_context ctx)`
    Tuple {
        elts: Vec<Self>,
    },
    /// `Compare(expr left, cmpop* ops, expr* comparators)`
    Compare {
        left: Box<Self>,
        ops: CmpOp,
        comparators: Box<Self>,
    },
    /// `Attribute(expr value, identifier attr, expr_context ctx)`
    Attribute {
        value: Box<Self>,
        attr: &'input str,
    },
    /// `Call(expr func, expr* args, keyword* keywords)`
    Call {
        func: Box<Self>,
        args: Vec<Self>,
        keywords: Vec<Keyword<'input>>,
    },
    None,
    Ellipsis,
    Pass,
}

pub fn expr(input: &str) -> ParseResult<Expr> {
    let (mut input, mut e) = alt((
        tag("None").map(|_| Expr::None),
        tag("...").map(|_| Expr::Ellipsis),
        tag("pass").map(|_| Expr::Pass),
        constant.map(|value| Expr::Constant { value }),
        identifier.map(|id| Expr::Name { id }),
        expr_tuple.map(|elts| Expr::Tuple { elts }),
    ))
    .parse(input)?;
    loop {
        let (input_new, comparators) = opt(tuple((multispace0, cmpop, multispace0, expr))
            .map(|(_sp1, ops, _sp2, comparators)| (ops, comparators)))
        .parse(input)?;
        if let Some((ops, comparators)) = comparators {
            input = input_new;
            e = Expr::Compare {
                left: Box::new(e),
                ops,
                comparators: Box::new(comparators),
            };
            continue;
        }

        let (input_new, attr) = opt(tuple((multispace0, char('.'), multispace0, identifier))
            .map(|(_sp1, _dot, _sp2, attr)| attr))
        .parse(input)?;
        if let Some(attr) = attr {
            input = input_new;
            e = Expr::Attribute {
                value: Box::new(e),
                attr,
            };
            continue;
        }

        let (input_new, call_args) =
            opt(tuple((multispace0, function_args)).map(|(_sp, args)| args)).parse(input)?;
        if let Some((args, keywords)) = call_args {
            input = input_new;
            e = Expr::Call {
                func: Box::new(e),
                args,
                keywords,
            };
            continue;
        }

        return Ok((input, e));
    }
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant<'input> {
    String(&'input str),
    Float(f64),
    Int(i64),
}

pub fn constant(input: &str) -> ParseResult<Constant> {
    alt((
        double.map(|f| Constant::Float(f)),
        string.map(|s| Constant::String(s)),
    ))
    .parse(input)
}

pub fn expr_tuple(input: &str) -> ParseResult<Vec<Expr>> {
    let (input, _open) = char('(').parse(input)?;
    let (input, _sp) = multispace0(input)?;

    let (input, inner) =
        separated_list0(tuple((multispace0, char(','), multispace0)), expr).parse(input)?;

    let (input, _sp) = multispace0(input)?;
    let (input, _close) = char(')').parse(input)?;
    Ok((input, inner))
}

/// `keyword = (identifier? arg, expr value)`
///
/// keyword arguments supplied to call (NULL identifier for `**kwargs`)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Keyword<'input> {
    arg: Option<&'input str>,
    value: Expr<'input>,
}

pub fn keyword(input: &str) -> ParseResult<Keyword> {
    let (input, star) = opt(tag("**")).parse(input)?;

    if let Some(_star) = star {
        let (input, value) = expr(input)?;
        return Ok((input, Keyword { arg: None, value }));
    }

    let (input, arg) = identifier(input)?;
    let (input, _) = tuple((multispace0, char('='), multispace0)).parse(input)?;
    let (input, value) = expr(input)?;

    Ok((
        input,
        Keyword {
            arg: Some(arg),
            value,
        },
    ))
}

enum Arg<'input> {
    Keyword(Keyword<'input>),
    Positional(Expr<'input>),
}

fn arg(input: &str) -> ParseResult<Arg> {
    alt((keyword.map(Arg::Keyword), expr.map(Arg::Positional))).parse(input)
}

pub fn function_args(input: &str) -> ParseResult<(Vec<Expr>, Vec<Keyword>)> {
    let (input, _open) = char('(').parse(input)?;
    let (input, _sp) = multispace0(input)?;

    let (input, inner) =
        separated_list0(tuple((multispace0, char(','), multispace0)), arg).parse(input)?;

    let (input, _sp) = multispace0(input)?;
    let (input, _close) = char(')').parse(input)?;

    let mut positional = Vec::new();
    let mut keywords = Vec::new();
    for arg in inner {
        match arg {
            Arg::Keyword(keyword) => keywords.push(keyword),
            Arg::Positional(value) => positional.push(value),
        }
    }

    Ok((input, (positional, keywords)))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;

    #[test]
    fn test_keyword() {
        insta::assert_debug_snapshot!(keyword("a=None").finish().unwrap());
        insta::assert_debug_snapshot!(keyword("**dict").finish().unwrap());
    }

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

    #[test]
    fn test_expr() {
        // Name
        insta::assert_debug_snapshot!(expr("a").finish().unwrap());

        // Attribute
        insta::assert_debug_snapshot!(expr("m.a.b").finish().unwrap());

        // Constant
        insta::assert_debug_snapshot!(expr("1.0").finish().unwrap());

        // Tuples
        insta::assert_debug_snapshot!(expr("()").finish().unwrap()); // zero-sized tuple
        insta::assert_debug_snapshot!(expr("(a, 1.0)").finish().unwrap());

        // Compare
        insta::assert_debug_snapshot!(expr("a < b").finish().unwrap());
        insta::assert_debug_snapshot!(expr("a < b < c").finish().unwrap()); // (< a (< b c))

        // Call
        insta::assert_debug_snapshot!(expr("f()").finish().unwrap());
        insta::assert_debug_snapshot!(expr("f(1, 2)").finish().unwrap());
        insta::assert_debug_snapshot!(expr("f(1, a = 2)").finish().unwrap());
        insta::assert_debug_snapshot!(expr(r#"f(1, "test")"#).finish().unwrap());
        insta::assert_debug_snapshot!(expr("None ()").finish().unwrap()); // This should be parsed as function call

        // Combinations
        insta::assert_debug_snapshot!(expr("f(1).g.h(2, 3)").finish().unwrap());
        insta::assert_debug_snapshot!(expr("sys.version_info >= (3, 9)").finish().unwrap());
    }
}
