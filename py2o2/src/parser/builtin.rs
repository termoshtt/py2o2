use super::*;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::*,
    multi::{many0, separated_list1},
    sequence::tuple,
    Parser,
};

pub fn docstring(input: &str) -> ParseResult<&str> {
    let (input, _start) = tag(r#"""""#).parse(input)?;
    let (input, doc) = take_until(r#"""""#).parse(input)?;
    let (input, _end) = tag(r#"""""#).parse(input)?;
    Ok((input, doc))
}

fn ident(input0: &str) -> ParseResult<&str> {
    // TODO: Support more unicode
    // https://docs.python.org/ja/3/reference/lexical_analysis.html#identifiers
    let alpha_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
    let alphanum_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
    let (input, _head) = alpha_1(input0)?;
    let (_input, tail) = many0(alphanum_1).parse(input)?;
    let n = tail.len() + 1;
    Ok((&input0[n..], &input0[..n]))
}

/// Builtin identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier<'input> {
    components: Vec<&'input str>,
}

pub fn identifier(input: &str) -> ParseResult<Identifier> {
    let (input, components) =
        separated_list1(tuple((multispace0, char('.'), multispace0)), ident).parse(input)?;
    Ok((input, Identifier { components }))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;

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
}
