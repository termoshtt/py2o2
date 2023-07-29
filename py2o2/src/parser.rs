//! Parse Python type stub file (*.pyi) generated by pyright
//!
//! The grammar of stub file is same as Python itself, described at
//! <https://docs.python.org/3/library/ast.html#abstract-grammar>

mod builtin;
mod expr;
mod stmt;

pub use builtin::*;
pub use expr::*;
pub use stmt::*;

use anyhow::{bail, Context, Result};
use nom::{character::complete::*, multi::separated_list0, Parser};
use std::{
    path::{self, PathBuf},
    process::Command,
};

pub type ParseResult<'input, T> = nom::IResult<&'input str, T>;

#[derive(Debug, PartialEq, Eq)]
pub struct AST<'input> {
    import: Vec<Import<'input>>,
    import_from: Vec<ImportFrom<'input>>,
    function_def: Vec<FunctionDef<'input>>,
}

pub fn parse(input: &str) -> ParseResult<Vec<Stmt>> {
    separated_list0(multispace1, stmt).parse(input)
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
