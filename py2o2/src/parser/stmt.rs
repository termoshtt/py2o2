use super::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::opt,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    Parser,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Type<'input> {
    Name(&'input str),
    None,
}

pub fn type_(input: &str) -> ParseResult<Type> {
    // FIXME: More possible types e.g. `Callable`
    let (input, name) = ident(input)?;
    Ok((input, Type::Name(name)))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arg<'input> {
    pub name: &'input str,
    pub ty: Option<Type<'input>>,
    pub default: Option<Expr>,
}

pub fn arg(input: &str) -> ParseResult<Arg> {
    let (input, (name, ty, default)) = tuple((
        ident,
        opt(tuple((multispace0, char(':'), multispace0, type_)).map(|(_sp1, _colon, _sp2, ty)| ty)),
        opt(tuple((multispace0, char('='), multispace0, expr))
            .map(|(_sp1, _colon, _sp2, default)| default)),
    ))
    .parse(input)?;
    Ok((input, Arg { name, ty, default }))
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDef<'input> {
    name: &'input str,
    args: Vec<Arg<'input>>,
    type_: Type<'input>,
}

pub fn function_def(input: &str) -> ParseResult<FunctionDef> {
    let (input, _def) = tag("def").parse(input)?;
    let (input, _space) = multispace1(input)?;
    let (input, name) = ident(input)?;
    let (input, args) = delimited(
        char('('),
        separated_list0(tuple((multispace0, char(','), multispace0)), arg),
        char(')'),
    )
    .parse(input)?;
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

#[derive(Debug, PartialEq, Eq)]
pub struct Alias<'input> {
    name: Path<'input>,
    asname: Option<&'input str>,
}

pub fn alias(input: &str) -> ParseResult<Alias> {
    let (input, name) = path(input)?;
    let (input, asname) =
        opt(tuple((multispace1, tag("as"), multispace1, ident))
            .map(|(_sp1, _as, _sp2, asname)| asname))
        .parse(input)?;
    Ok((input, Alias { name, asname }))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import<'input> {
    names: Vec<Alias<'input>>,
}

pub fn import(input: &str) -> ParseResult<Import> {
    let (input, _import) = tag("import").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, names) =
        separated_list1(tuple((multispace0, char(','), multispace0)), alias).parse(input)?;
    Ok((input, Import { names }))
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportFrom<'input> {
    module: Path<'input>,
    names: Vec<Alias<'input>>,
}

pub fn import_from(input: &str) -> ParseResult<ImportFrom> {
    let (input, _from) = tag("from").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, module) = path(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, _import) = tag("import").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, names) =
        separated_list1(tuple((multispace0, char(','), multispace0)), alias).parse(input)?;
    Ok((input, ImportFrom { module, names }))
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt<'input> {
    ModuleDoc(&'input str),
    Import(Import<'input>),
    ImportFrom(ImportFrom<'input>),
    FunctionDef(FunctionDef<'input>),
    If(If<'input>),
}

pub fn stmt(input: &str) -> ParseResult<Stmt> {
    alt((
        docstring.map(Stmt::ModuleDoc),
        import.map(Stmt::Import),
        import_from.map(Stmt::ImportFrom),
        function_def.map(Stmt::FunctionDef),
        if_.map(Stmt::If),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct If<'input> {
    test: Expr,
    body: Box<Stmt<'input>>,
    orelse: Option<Box<Stmt<'input>>>,
}

pub fn if_(input: &str) -> ParseResult<If> {
    let (input, _if) = tuple((tag("if"), multispace1)).parse(input)?;
    let (input, test) = expr(input)?;
    let (input, _colon) = tuple((multispace0, char(':'), multispace0))(input)?;
    let (input, body) = stmt.map(Box::new).parse(input)?;
    let (input, orelse) = opt(tuple((
        multispace1,
        tag("else"),
        multispace0,
        char(':'),
        multispace0,
        stmt,
    ))
    .map(|(_sp1, _else, _sp2, _colon, _sp3, orelse)| Box::new(orelse)))
    .parse(input)?;
    Ok((input, If { test, body, orelse }))
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
        insta::assert_debug_snapshot!(function_def(
            r#"
            def g(x: int) -> str:
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());
    }

    #[test]
    fn parse_import() {
        insta::assert_debug_snapshot!(import("import numpy, pandas").finish().unwrap());
        insta::assert_debug_snapshot!(import_from("from numpy import array").finish().unwrap());
    }

    #[test]
    fn parse_if() {
        insta::assert_debug_snapshot!(if_(r#"
            if sys.version_info >= (3, 9):
                ...
            "#
        .trim())
        .finish()
        .unwrap());
    }
}
