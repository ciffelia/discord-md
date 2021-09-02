//! Useful parser functions written with [`nom`]

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::anychar,
    combinator::{peek, recognize, rest, verify},
    error::Error,
    multi::many_till,
    sequence::delimited,
    Compare, FindToken, IResult, InputLength, InputTake, InputTakeAtPosition, Parser,
};

/// Return the remaining input.
///
/// This parser is similar to [`nom::combinator::rest`], but returns `Err(Err::Error((_, ErrorKind::Verify)))` if the input is empty.
pub fn rest1(s: &str) -> IResult<&str, &str> {
    verify(rest, |x: &str| !x.is_empty())(s)
}

/// Gets an object sandwiched in a pattern.
///
/// Returns `Err(Err::Error((_, ErrorKind::Tag)))` if the input doesn't match the pattern (i.e. sandwich doesn't start),
/// or matches the pattern only once (i.e. sandwich doesn't end).
///
/// Returns `Err(Err::Error((_, ErrorKind::IsNot)))` if the input matches pattern twice
/// but no object are found between two patterns (i.e. no sandwich fillings are found).
pub fn wrapped<Input, W>(wrapper: W) -> impl FnMut(Input) -> IResult<Input, Input>
where
    Input: InputTake + InputTakeAtPosition + Compare<W>,
    W: InputLength + FindToken<<Input as InputTakeAtPosition>::Item> + Clone,
{
    delimited(tag(wrapper.clone()), is_not(wrapper.clone()), tag(wrapper))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::test_util::parse_error;
    use nom::error::ErrorKind;

    #[test]
    fn test_rest1() {
        assert_eq!(rest1("hello"), Ok(("", "hello")));
        assert_eq!(rest1(""), Err(parse_error("", ErrorKind::Verify)));
    }

    #[test]
    fn test_wrapped() {
        let mut parser = wrapped("*");

        assert_eq!(parser("*hello*"), Ok(("", "hello")));
        assert_eq!(parser("*hello*world"), Ok(("world", "hello")));
        assert_eq!(parser("*hello"), Err(parse_error("", ErrorKind::Tag)));
        assert_eq!(parser("hello*"), Err(parse_error("hello*", ErrorKind::Tag)));
        assert_eq!(parser(""), Err(parse_error("", ErrorKind::Tag)));
        assert_eq!(parser("**"), Err(parse_error("*", ErrorKind::IsNot)));
    }

    #[test]
    fn test_take_before0() {
        let mut parser = take_before0(tag("end"));

        assert_eq!(parser("123end456"), Ok(("end456", "123")));
        assert_eq!(parser("end456"), Ok(("end456", "")));
        assert_eq!(parser("123"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(parser(""), Err(parse_error("", ErrorKind::Eof)));
    }
}
