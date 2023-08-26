//! [nom](https://docs.rs/nom/)-based parser of Python AST
//!
//! Abstract syntax tree of Python is defined at <https://docs.python.org/3/library/ast.html>
//!

mod builtin;
mod expr;
mod function_def;

pub use builtin::*;
pub use expr::*;
pub use function_def::*;

pub type ParseResult<'input, T> = nom::IResult<&'input str, T>;

#[cfg(test)]
mod test {
    use super::ParseResult;
    use nom::Finish;

    /// Helper for integrating with insta snapshot testing
    pub trait CheckParsed: Sized {
        type Inner;
        fn check_parsed(self) -> Self::Inner;
    }
    impl<T> CheckParsed for ParseResult<'_, T> {
        type Inner = T;
        fn check_parsed(self) -> Self::Inner {
            let (input, out) = self.finish().unwrap();
            assert!(input.is_empty());
            out
        }
    }
}
