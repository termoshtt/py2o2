//! [nom](https://docs.rs/nom/)-based parser of Python AST
//!
//! Abstract syntax tree of Python is defined at <https://docs.python.org/3/library/ast.html>
//!

mod builtin;
mod expr;

pub use builtin::*;
pub use expr::*;

pub type ParseResult<'input, T> = nom::IResult<&'input str, T>;
