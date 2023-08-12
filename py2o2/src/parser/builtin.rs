use super::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::*,
    multi::many0,
    Parser,
};

pub fn multiline_string_literal(input: &str) -> ParseResult<&str> {
    let (input, _start) = tag(r#"""""#).parse(input)?;
    let (input, doc) = take_until(r#"""""#).parse(input)?;
    let (input, _end) = tag(r#"""""#).parse(input)?;
    Ok((input, doc))
}

pub fn string_literal(input: &str) -> ParseResult<&str> {
    let (input, _start) = char('"').parse(input)?;
    let (input, doc) = take_until(r#"""#).parse(input)?;
    let (input, _end) = char('"').parse(input)?;
    Ok((input, doc))
}

pub fn string(input: &str) -> ParseResult<&str> {
    alt((multiline_string_literal, string_literal)).parse(input)
}

pub fn identifier(input0: &str) -> ParseResult<&str> {
    // TODO: Support more unicode
    // https://docs.python.org/ja/3/reference/lexical_analysis.html#identifiers
    let alpha_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
    let alphanum_1 = satisfy(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
    let (input, _head) = alpha_1(input0)?;
    let (_input, tail) = many0(alphanum_1).parse(input)?;
    let n = tail.len() + 1;
    Ok((&input0[n..], &input0[..n]))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;

    #[test]
    fn parse_ident() {
        assert_eq!(identifier("abc").finish().unwrap(), ("", "abc"));
        assert_eq!(identifier("abc0").finish().unwrap(), ("", "abc0"));
        assert_eq!(identifier("abc def").finish().unwrap(), (" def", "abc"));

        assert!(identifier("0abc").finish().is_err());
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            string(r#""""document""""#).finish().unwrap(),
            ("", "document")
        );
    }
}
