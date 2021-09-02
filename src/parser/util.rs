//! Useful parser functions written with [`nom`]

use nom::{
    character::complete::anychar,
    combinator::{peek, recognize, rest, verify},
    error::Error,
    multi::many_till,
    IResult, Parser,
};

/// Return the remaining input.
///
/// This parser is similar to [`nom::combinator::rest`], but returns `Err(Err::Error((_, ErrorKind::Verify)))` if the input is empty.
pub fn rest1(s: &str) -> IResult<&str, &str> {
    verify(rest, |x: &str| !x.is_empty())(s)
}

/// Returns the *shortest* input slice until it matches a parser.
///
/// Returns `Err(Err::Error((_, ErrorKind::Eof)))` if the input doesn't match the parser.
pub fn take_before0<'a, FOutput, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str>
where
    F: Parser<&'a str, FOutput, Error<&'a str>>,
{
    recognize(many_till(anychar, peek(f)))
}

/// Returns the *shortest* input slice until it matches a parser.
///
/// This parser is similar to [`take_before0`], but must return at least one character.
///
/// Returns `Err(Err::Error((_, ErrorKind::Eof)))` if the input doesn't match the parser.
///
/// Returns `Err(Err::Error((_, ErrorKind::Verify)))` if the input itself matches the parser
/// (i.e. this parser cannot return any characters).
pub fn take_before1<'a, FOutput, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str>
where
    F: Parser<&'a str, FOutput, Error<&'a str>>,
{
    verify(take_before0(f), |x: &str| !x.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::test_util::parse_error;
    use nom::bytes::complete::tag;
    use nom::error::ErrorKind;

    #[test]
    fn test_rest1() {
        assert_eq!(rest1("hello"), Ok(("", "hello")));
        assert_eq!(rest1(""), Err(parse_error("", ErrorKind::Verify)));
    }

    #[test]
    fn test_take_before0() {
        let mut parser = take_before0(tag("end"));

        assert_eq!(parser("123end456"), Ok(("end456", "123")));
        assert_eq!(parser("end456"), Ok(("end456", "")));
        assert_eq!(parser("123"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(parser(""), Err(parse_error("", ErrorKind::Eof)));
    }

    #[test]
    fn test_take_before1() {
        let mut parser = take_before1(tag("end"));

        assert_eq!(parser("123end456"), Ok(("end456", "123")));
        assert_eq!(
            parser("end456"),
            Err(parse_error("end456", ErrorKind::Verify))
        );
        assert_eq!(parser("123"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(parser(""), Err(parse_error("", ErrorKind::Eof)));
    }
}
