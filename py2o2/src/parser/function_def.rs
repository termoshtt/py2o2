use super::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, tuple},
    Parser,
};
use syn::parse::Parse;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Type<'input> {
    Name(&'input str),
    None,
}

pub fn type_(input: &str) -> ParseResult<Type> {
    // FIXME: More possible types e.g. `Callable`
    let (input, name) = identifier(input)?;
    Ok((input, Type::Name(name)))
}

/// `arg = (identifier arg, expr? annotation, string? type_comment)`
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Arg<'input> {
    pub name: &'input str,
    pub ty: Option<Type<'input>>,
    pub default: Option<Expr<'input>>,
}

pub fn arg(input: &str) -> ParseResult<Arg> {
    let (input, (name, ty, default)) = tuple((
        identifier,
        opt(tuple((multispace0, char(':'), multispace0, type_)).map(|(_sp1, _colon, _sp2, ty)| ty)),
        opt(tuple((multispace0, char('='), multispace0, expr))
            .map(|(_sp1, _colon, _sp2, default)| default)),
    ))
    .parse(input)?;
    Ok((input, Arg { name, ty, default }))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ArgLike<'input> {
    /// Usual argument
    Arg(Arg<'input>),
    /// `*args`
    VarArg(Arg<'input>),
    /// `**kwds`
    KwArg(Arg<'input>),
    /// `/`
    PositionalSep,
    /// `*`
    KeywordSep,
}

pub fn arg_like(input: &str) -> ParseResult<ArgLike> {
    alt((
        char('/').map(|_| ArgLike::PositionalSep),
        tuple((char('*'), multispace0, arg)).map(|(_star, _sp, arg)| ArgLike::VarArg(arg)),
        tuple((tag("**"), multispace0, arg)).map(|(_star, _sp, arg)| ArgLike::KwArg(arg)),
        char('*').map(|_| ArgLike::KeywordSep),
        arg.map(ArgLike::Arg),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Arguments<'input> {
    /// Usual arguments
    args: Vec<Arg<'input>>,
    /// Args before `/`
    positional_only: Vec<Arg<'input>>,
    /// Args after `*`
    keyword_only: Vec<Arg<'input>>,
    /// `*arg` will be stored without `*`
    var_args: Option<Arg<'input>>,
    /// `**kwargs` will be stored without `**`
    kw_args: Option<Arg<'input>>,
}

impl<'input> From<Vec<ArgLike<'input>>> for Arguments<'input> {
    fn from(args: Vec<ArgLike<'input>>) -> Self {
        let mut arguments = Self::default();
        let mut cursor = &mut arguments.args;
        for arg in args {
            match arg {
                ArgLike::Arg(arg) => cursor.push(arg),
                ArgLike::PositionalSep => arguments.positional_only.append(cursor),
                ArgLike::KeywordSep => cursor = &mut arguments.keyword_only,
                ArgLike::KwArg(arg) => arguments.kw_args = Some(arg),
                ArgLike::VarArg(arg) => arguments.var_args = Some(arg),
            }
        }
        arguments
    }
}

pub fn arguments(input: &str) -> ParseResult<Arguments> {
    let (input, args) =
        separated_list0(tuple((multispace0, char(','), multispace0)), arg_like).parse(input)?;
    Ok((input, args.into()))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FunctionDef<'input> {
    name: &'input str,
    args: Arguments<'input>,
    type_: Type<'input>,
}

pub fn function_def(input: &str) -> ParseResult<FunctionDef> {
    let (input, _def) = tag("def").parse(input)?;
    let (input, _space) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, args) = delimited(char('('), arguments, char(')')).parse(input)?;
    let (input, ty) =
        opt(tuple((multispace0, tag("->"), multispace0, type_)).map(|(_sp1, _arrow, _sp2, ty)| ty))
            .parse(input)?;
    let (input, _colon) = char(':').parse(input)?;
    let (input, _sp) = multispace0(input)?;
    let (input, _expr) = expr(input)?;
    Ok((
        input,
        FunctionDef {
            name,
            args,
            type_: ty.unwrap_or(Type::None),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;

    #[test]
    fn parse_arg() {
        insta::assert_debug_snapshot!(arg("a").finish().unwrap());
        insta::assert_debug_snapshot!(arg("a: T").finish().unwrap());
        insta::assert_debug_snapshot!(arg("a: T = None").finish().unwrap());
        insta::assert_debug_snapshot!(arg("a = None").finish().unwrap());
    }

    #[test]
    fn parse_function_def() {
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(a, b):
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        // type hint
        insta::assert_debug_snapshot!(function_def(
            r#"
            def g(x: int) -> str:
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        // *arg
        insta::assert_debug_snapshot!(function_def(
            r#"
            def g(*args) -> str:
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        // **keywords
        insta::assert_debug_snapshot!(function_def(
            r#"
            def g(**keywords) -> str:
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        // Positional only
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(x, /, a=1):
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        // Keyword only
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(x, *, a=1):
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        // Combined
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(x, /, y, *, a=1):
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());
    }
}
