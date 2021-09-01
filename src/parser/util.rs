//! Useful parsers

use nom::{
    bytes::complete::{is_not, tag, take_while1},
    character::complete::anychar,
    combinator::{peek, recognize},
    error::Error,
    lib::std::ops::{RangeFrom, RangeTo},
    multi::many_till,
    sequence::delimited,
    AsChar, Compare, FindToken, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition,
    Offset, Parser, Slice,
};

/// Return the remaining input.
///
/// This parser is similar to [`nom::combinator::rest`], but returns `Err(Err::Error((_, ErrorKind::TakeWhile1)))` if the input is empty.
pub fn rest1(s: &str) -> IResult<&str, &str> {
    take_while1(|_| true)(s)
}

/// Gets an object sandwiched by a pattern.
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
pub fn take_before<Input, FOutput, F>(f: F) -> impl FnMut(Input) -> IResult<Input, Input>
where
    Input:
        InputIter + InputLength + Offset + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>> + Clone,
    <Input as InputIter>::Item: AsChar,
    F: Parser<Input, FOutput, Error<Input>>,
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
        assert_eq!(rest1(""), Err(parse_error("", ErrorKind::TakeWhile1)));
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
    fn test_take_before() {
        let mut parser = take_before(tag("end"));

        assert_eq!(parser("123end456"), Ok(("end456", "123")));
        assert_eq!(parser("end456"), Ok(("end456", "")));
        assert_eq!(parser("123"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(parser(""), Err(parse_error("", ErrorKind::Eof)));
    }
}
