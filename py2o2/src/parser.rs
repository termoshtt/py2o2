//! Parse Python type stub file (*.pyi) generated by pyright
//!
//! The grammar of stub file is same as Python itself, descripted at
//! https://docs.python.org/3/library/ast.html#abstract-grammar

use anyhow::{bail, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::opt,
    multi::many0,
    sequence::{delimited, tuple},
    Parser,
};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Stub {}

type ParseResult<'input, T> = nom::IResult<&'input str, T>;

fn ident(input0: &str) -> ParseResult<&str> {
    let alpha_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
    let alphanum_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
    let (input, head) = alpha_1(input0)?;
    let (input, tail) = many0(alphanum_1).parse(input)?;
    let n = tail.len() + 1;
    Ok((&input0[n..], &input0[..n]))
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    None,
    Ellipsis,
    Pass,
}

fn expr(input: &str) -> ParseResult<Expr> {
    alt((
        tag("None").map(|_| Expr::None),
        tag("...").map(|_| Expr::Ellipsis),
        tag("pass").map(|_| Expr::Pass),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Arg<'input> {
    name: &'input str,
    ty: Option<&'input str>,
    default: Option<Expr>,
}

fn arg(input: &str) -> ParseResult<Arg> {
    let (input, (name, ty, default)) = tuple((
        ident,
        opt(tuple((multispace0, char(':'), multispace0, ident)).map(|(_sp1, _colon, _sp2, ty)| ty)),
        opt(tuple((multispace0, char('='), multispace0, expr))
            .map(|(_sp1, _colon, _sp2, default)| default)),
    ))
    .parse(input)?;
    Ok((input, Arg { name, ty, default }))
}

pub fn parse(pyi_input: &str) -> Result<Stub> {
    todo!("{}", pyi_input)
}

pub fn generate_pyi(target: &str, root: &Path) -> Result<PathBuf> {
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
            return Ok(dest);
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
    fn parse_arg() {
        assert_eq!(
            arg("a").finish().unwrap(),
            (
                "",
                Arg {
                    name: "a",
                    ty: None,
                    default: None
                }
            )
        );
        assert_eq!(
            arg("a: T").finish().unwrap(),
            (
                "",
                Arg {
                    name: "a",
                    ty: Some("T"),
                    default: None
                }
            )
        );
        assert_eq!(
            arg("a: T = None").finish().unwrap(),
            (
                "",
                Arg {
                    name: "a",
                    ty: Some("T"),
                    default: Some(Expr::None)
                }
            )
        );
        assert_eq!(
            arg("a = None").finish().unwrap(),
            (
                "",
                Arg {
                    name: "a",
                    ty: None,
                    default: Some(Expr::None)
                }
            )
        );
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn parse_numpy_init() -> anyhow::Result<()> {
        let numpy_typing = generate_pyi("numpy", &repo_root())?;
        let pyi = fs::read_to_string(numpy_typing.join("__init__.pyi"))?;
        let _stub = parse(&pyi)?;
        Ok(())
    }
}
