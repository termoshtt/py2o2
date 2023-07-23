//! Parse Python type stub file (*.pyi) generated by pyright
//!
//! The grammar of stub file is same as Python itself, described at
//! https://docs.python.org/3/library/ast.html#abstract-grammar

use anyhow::{bail, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::*,
    combinator::opt,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    Parser,
};
use std::{
    path::{self, PathBuf},
    process::Command,
};

pub type ParseResult<'input, T> = nom::IResult<&'input str, T>;

pub fn docstring(input: &str) -> ParseResult<&str> {
    let (input, _start) = tag(r#"""""#).parse(input)?;
    let (input, doc) = take_until(r#"""""#).parse(input)?;
    let (input, _end) = tag(r#"""""#).parse(input)?;
    Ok((input, doc))
}

pub fn ident(input0: &str) -> ParseResult<&str> {
    // TODO: Support more unicode
    // https://docs.python.org/ja/3/reference/lexical_analysis.html#identifiers
    let alpha_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
    let alphanum_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
    let (input, _head) = alpha_1(input0)?;
    let (_input, tail) = many0(alphanum_1).parse(input)?;
    let n = tail.len() + 1;
    Ok((&input0[n..], &input0[..n]))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<'input> {
    components: Vec<&'input str>,
}

pub fn path(input: &str) -> ParseResult<Path> {
    let (input, components) =
        separated_list1(tuple((multispace0, char('.'), multispace0)), ident).parse(input)?;
    Ok((input, Path { components }))
}

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
}

pub fn statement(input: &str) -> ParseResult<Stmt> {
    alt((
        docstring.map(Stmt::ModuleDoc),
        import.map(Stmt::Import),
        import_from.map(Stmt::ImportFrom),
        function_def.map(Stmt::FunctionDef),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct AST<'input> {
    import: Vec<Import<'input>>,
    import_from: Vec<ImportFrom<'input>>,
    function_def: Vec<FunctionDef<'input>>,
}

pub fn parse(input: &str) -> ParseResult<Vec<Stmt>> {
    separated_list0(multispace1, statement).parse(input)
}

pub fn generate_pyi(target: &str, root: &path::Path) -> Result<PathBuf> {
    let dest = root.join("typings").join(target);
    if dest.exists() {
        return Ok(dest);
    }

    let out = Command::new("pyright")
        .arg("--createstub")
        .arg(target)
        .current_dir(root)
        .output()
        .with_context(|| "pyright is not found")?;
    if out.status.success() {
        if dest.exists() {
            Ok(dest)
        } else {
            bail!(
                "pyright does not creates {}. Something wrong.",
                dest.display()
            );
        }
    } else {
        bail!(
            "pyright exit with error: {}",
            std::str::from_utf8(&out.stderr).unwrap_or("Non UTF-8 error message")
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;
    use std::fs;

    #[test]
    fn parse_ident() {
        assert_eq!(ident("abc").finish().unwrap(), ("", "abc"));
        assert_eq!(ident("abc0").finish().unwrap(), ("", "abc0"));
        assert_eq!(ident("abc def").finish().unwrap(), (" def", "abc"));

        assert!(ident("0abc").finish().is_err());
    }

    #[test]
    fn parse_docstring() {
        assert_eq!(
            docstring(r#""""document""""#).finish().unwrap(),
            ("", "document")
        );
    }

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

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[ignore]
    #[test]
    fn parse_numpy_init_pyi() {
        let numpy_typing = generate_pyi("numpy", &repo_root()).unwrap();
        let pyi = fs::read_to_string(numpy_typing.join("__init__.pyi")).unwrap();
        let (res, _stmt) = parse(&pyi).finish().unwrap();
        for line in res.lines().take(5) {
            eprintln!("{}", line);
        }
        eprintln!("... and more lines");
        assert!(res.is_empty());
    }
}
