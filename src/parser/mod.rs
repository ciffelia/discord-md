//! Markdown parser functions written with [`nom`]

mod util;

#[cfg(test)]
mod test_util;

use crate::ast::{
    Bold, ItalicsStar, ItalicsUnderscore, MarkdownDocument, MarkdownElement,
    MarkdownElementCollection, MultiLineCode, OneLineCode, Plain, Spoiler, Strikethrough,
    Underline,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    combinator::{map, map_parser, opt, peek, rest},
    multi::many0,
    sequence::{delimited, pair, terminated},
    IResult,
};
use util::{rest1, take_before0, take_before1};

/// Parses a markdown document.
pub fn markdown_document(i: &str) -> IResult<&str, MarkdownDocument> {
    map(markdown_element_collection, MarkdownDocument::new)(i)
}

/// Parses a collection of markdown element.
fn markdown_element_collection(i: &str) -> IResult<&str, MarkdownElementCollection> {
    map(many0(markdown_element), MarkdownElementCollection::from)(i)
}

/// Parses a markdown element.
fn markdown_element(i: &str) -> IResult<&str, MarkdownElement> {
    alt((markdown_element_not_plain, markdown_element_plain))(i)
}

/// Parses a plain markdown element.
fn markdown_element_plain(i: &str) -> IResult<&str, MarkdownElement> {
    map(plain, MarkdownElement::from)(i)
}

/// Parses a styled markdown element.
fn markdown_element_not_plain(i: &str) -> IResult<&str, MarkdownElement> {
    alt((
        map(multi_line_code, MarkdownElement::from),
        map(one_line_code, MarkdownElement::from),
        map(italics_star, MarkdownElement::from),
        map(italics_underscore, MarkdownElement::from),
        map(bold, MarkdownElement::from),
        map(underline, MarkdownElement::from),
        map(strikethrough, MarkdownElement::from),
        map(spoiler, MarkdownElement::from),
    ))(i)
}

/// Parses plain text.
fn plain(i: &str) -> IResult<&str, Plain> {
    map(
        alt((take_before0(markdown_element_not_plain), rest1)),
        Plain::new,
    )(i)
}

/// Parses italics text wrapped in `*`.
fn italics_star(i: &str) -> IResult<&str, ItalicsStar> {
    map(
        map_parser(
            delimited(tag("*"), take_before1(tag("*")), tag("*")),
            markdown_element_collection,
        ),
        ItalicsStar::new,
    )(i)
}

/// Parses italics text wrapped in `_`.
fn italics_underscore(i: &str) -> IResult<&str, ItalicsUnderscore> {
    map(
        map_parser(
            delimited(tag("_"), take_before1(tag("_")), tag("_")),
            markdown_element_collection,
        ),
        ItalicsUnderscore::new,
    )(i)
}

/// Parses bold text.
fn bold(i: &str) -> IResult<&str, Bold> {
    map(
        map_parser(
            delimited(tag("**"), take_before1(tag("**")), tag("**")),
            markdown_element_collection,
        ),
        Bold::new,
    )(i)
}

/// Parses underline text.
fn underline(i: &str) -> IResult<&str, Underline> {
    map(
        map_parser(
            delimited(tag("__"), take_before1(tag("__")), tag("__")),
            markdown_element_collection,
        ),
        Underline::new,
    )(i)
}

/// Parses strikethrough text.
fn strikethrough(i: &str) -> IResult<&str, Strikethrough> {
    map(
        map_parser(
            delimited(tag("~~"), take_before1(tag("~~")), tag("~~")),
            markdown_element_collection,
        ),
        Strikethrough::new,
    )(i)
}

/// Parses spoiler text.
fn spoiler(i: &str) -> IResult<&str, Spoiler> {
    map(
        map_parser(
            delimited(tag("||"), take_before1(tag("||")), tag("||")),
            markdown_element_collection,
        ),
        Spoiler::new,
    )(i)
}

/// Parses an inline code block.
fn one_line_code(i: &str) -> IResult<&str, OneLineCode> {
    map(
        delimited(tag("`"), take_before1(tag("`")), tag("`")),
        OneLineCode::new,
    )(i)
}

/// Parses a multiline code block.
fn multi_line_code(i: &str) -> IResult<&str, MultiLineCode> {
    map(
        map_parser(
            delimited(tag("```"), take_before1(tag("```")), tag("```")),
            pair(opt(terminated(alphanumeric1, peek(newline))), rest),
        ),
        |(lang, content): (Option<&str>, &str)| {
            MultiLineCode::new(content, lang.map(|x| x.to_string()))
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::test_util::parse_error;
    use nom::error::ErrorKind;

    #[test]
    fn test_markdown_document() {
        assert_eq!(
            markdown_document("`hello`"),
            Ok((
                "",
                MarkdownDocument::new(vec![OneLineCode::new("hello").into()])
            ))
        );
        assert_eq!(
            markdown_document("**hello _world_**"),
            Ok((
                "",
                MarkdownDocument::new(vec![Bold::new(vec![
                    Plain::new("hello ").into(),
                    ItalicsUnderscore::new(vec![Plain::new("world").into()]).into()
                ])
                .into()])
            ))
        );
        // Note: `***italics* in bold**` works, but `***bold** in italics*` doesn't work.
        // This is a known limitation.
        assert_eq!(
            markdown_document("***italics* in bold**"),
            Ok((
                "",
                MarkdownDocument::new(vec![Bold::new(vec![
                    ItalicsStar::new(vec![Plain::new("italics").into()]).into(),
                    Plain::new(" in bold").into(),
                ])
                .into()])
            ))
        );
        // Note: `___italics_ in underline__` works, but `___underline__ in italics_` doesn't work.
        // This is a known limitation.
        assert_eq!(
            markdown_document("___italics_ in underline__"),
            Ok((
                "",
                MarkdownDocument::new(vec![Underline::new(vec![
                    ItalicsUnderscore::new(vec![Plain::new("italics").into()]).into(),
                    Plain::new(" in underline").into(),
                ])
                .into()])
            ))
        );
        assert_eq!(
            markdown_document(""),
            Ok(("", MarkdownDocument::new(vec![])))
        );
    }

    #[test]
    fn test_markdown_element_collection() {
        assert_eq!(
            markdown_element_collection("~~hello~~"),
            Ok((
                "",
                MarkdownElementCollection::new(vec![Strikethrough::new(vec![
                    Plain::new("hello").into()
                ])
                .into()])
            ))
        );
        assert_eq!(
            markdown_element_collection("**hello** _world_"),
            Ok((
                "",
                MarkdownElementCollection::new(vec![
                    Bold::new(vec![Plain::new("hello").into()]).into(),
                    Plain::new(" ").into(),
                    ItalicsUnderscore::new(vec![Plain::new("world").into()]).into()
                ])
            ))
        );
        assert_eq!(
            markdown_element_collection("`__hello__` ||world||"),
            Ok((
                "",
                MarkdownElementCollection::new(vec![
                    OneLineCode::new("__hello__").into(),
                    Plain::new(" ").into(),
                    Spoiler::new(vec![Plain::new("world").into()]).into()
                ])
            ))
        );
        assert_eq!(
            markdown_element_collection(""),
            Ok(("", MarkdownElementCollection::new(vec![])))
        );
    }

    #[test]
    fn test_markdown_element_ok() {
        assert_eq!(
            markdown_element("text"),
            Ok(("", Plain::new("text").into()))
        );
        assert_eq!(
            markdown_element("**text"),
            Ok(("", Plain::new("**text").into()))
        );
        assert_eq!(
            markdown_element("text__"),
            Ok(("", Plain::new("text__").into()))
        );

        let content = || vec![Plain::new("text").into()];
        assert_eq!(
            markdown_element("*text*"),
            Ok(("", ItalicsStar::new(content()).into()))
        );
        assert_eq!(
            markdown_element("_text_"),
            Ok(("", ItalicsUnderscore::new(content()).into()))
        );
        assert_eq!(
            markdown_element("**text**"),
            Ok(("", Bold::new(content()).into()))
        );
        assert_eq!(
            markdown_element("__text__"),
            Ok(("", Underline::new(content()).into()))
        );
        assert_eq!(
            markdown_element("~~text~~"),
            Ok(("", Strikethrough::new(content()).into()))
        );
        assert_eq!(
            markdown_element("||text||"),
            Ok(("", Spoiler::new(content()).into()))
        );

        assert_eq!(
            markdown_element("`text`"),
            Ok(("", OneLineCode::new("text").into()))
        );
        assert_eq!(
            markdown_element("```\ntext```"),
            Ok(("", MultiLineCode::new("\ntext", None).into()))
        );
        assert_eq!(
            markdown_element("```html\ntext```"),
            Ok((
                "",
                MultiLineCode::new("\ntext", Some("html".to_string())).into()
            ))
        );

        assert_eq!(
            markdown_element("hello**world**"),
            Ok(("**world**", Plain::new("hello").into()))
        );
        assert_eq!(
            markdown_element("`hello`**world**"),
            Ok(("**world**", OneLineCode::new("hello").into()))
        );
    }

    #[test]
    fn test_markdown_element_err() {
        assert_eq!(
            markdown_element(""),
            Err(parse_error("", ErrorKind::Verify))
        );
    }

    #[test]
    fn test_markdown_element_combined() {
        assert_eq!(
            markdown_element("__*text*__"),
            Ok((
                "",
                Underline::new(vec![
                    ItalicsStar::new(vec![Plain::new("text").into()]).into()
                ])
                .into()
            ))
        );
    }

    #[test]
    fn test_plain_ok() {
        assert_eq!(plain("text"), Ok(("", Plain::new("text"))));
        assert_eq!(
            plain("text *italics*"),
            Ok(("*italics*", Plain::new("text ")))
        );
        assert_eq!(plain("*italics*"), Ok(("*italics*", Plain::new(""))));
    }

    #[test]
    fn test_plain_err() {
        assert_eq!(plain(""), Err(parse_error("", ErrorKind::Verify)));
    }

    #[test]
    fn test_italics_star_ok() {
        assert_eq!(
            italics_star("*text*"),
            Ok(("", ItalicsStar::new(vec![Plain::new("text").into()])))
        );
    }

    #[test]
    fn test_italics_star_err() {
        assert_eq!(italics_star("*text"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(
            italics_star("text*"),
            Err(parse_error("text*", ErrorKind::Tag))
        );
        assert_eq!(
            italics_star("text"),
            Err(parse_error("text", ErrorKind::Tag))
        );
        assert_eq!(italics_star("**"), Err(parse_error("*", ErrorKind::Verify)));
    }

    #[test]
    fn test_italics_underscore_ok() {
        assert_eq!(
            italics_underscore("_text_"),
            Ok((
                "",
                ItalicsUnderscore::new(vec![Plain::new("text".to_string()).into()])
            ))
        );
    }

    #[test]
    fn test_italics_underscore_err() {
        assert_eq!(
            italics_underscore("_text"),
            Err(parse_error("", ErrorKind::Eof))
        );
        assert_eq!(
            italics_underscore("text_"),
            Err(parse_error("text_", ErrorKind::Tag))
        );
        assert_eq!(
            italics_underscore("text"),
            Err(parse_error("text", ErrorKind::Tag))
        );
        assert_eq!(
            italics_underscore("__"),
            Err(parse_error("_", ErrorKind::Verify))
        );
    }

    #[test]
    fn test_bold_ok() {
        assert_eq!(
            bold("**text**"),
            Ok(("", Bold::new(vec![Plain::new("text").into()])))
        );
    }

    #[test]
    fn test_bold_err() {
        assert_eq!(bold("**text"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(bold("text**"), Err(parse_error("text**", ErrorKind::Tag)));
        assert_eq!(bold("*text*"), Err(parse_error("*text*", ErrorKind::Tag)));
        assert_eq!(bold("text"), Err(parse_error("text", ErrorKind::Tag)));
        assert_eq!(bold("****"), Err(parse_error("**", ErrorKind::Verify)));
    }

    #[test]
    fn test_underline_ok() {
        assert_eq!(
            underline("__text__"),
            Ok(("", Underline::new(vec![Plain::new("text").into()])))
        );
    }

    #[test]
    fn test_underline_err() {
        assert_eq!(underline("__text"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(
            underline("text__"),
            Err(parse_error("text__", ErrorKind::Tag))
        );
        assert_eq!(
            underline("_text_"),
            Err(parse_error("_text_", ErrorKind::Tag))
        );
        assert_eq!(underline("text"), Err(parse_error("text", ErrorKind::Tag)));
        assert_eq!(underline("____"), Err(parse_error("__", ErrorKind::Verify)));
    }

    #[test]
    fn test_strikethrough_ok() {
        assert_eq!(
            strikethrough("~~text~~"),
            Ok(("", Strikethrough::new(vec![Plain::new("text").into()])))
        );
    }

    #[test]
    fn test_strikethrough_err() {
        assert_eq!(
            strikethrough("~~text"),
            Err(parse_error("", ErrorKind::Eof))
        );
        assert_eq!(
            strikethrough("text~~"),
            Err(parse_error("text~~", ErrorKind::Tag))
        );
        assert_eq!(
            strikethrough("~text~"),
            Err(parse_error("~text~", ErrorKind::Tag))
        );
        assert_eq!(
            strikethrough("text"),
            Err(parse_error("text", ErrorKind::Tag))
        );
        assert_eq!(
            strikethrough("~~~~"),
            Err(parse_error("~~", ErrorKind::Verify))
        );
    }

    #[test]
    fn test_spoiler_ok() {
        assert_eq!(
            spoiler("||text||"),
            Ok(("", Spoiler::new(vec![Plain::new("text").into()])))
        );
    }

    #[test]
    fn test_spoiler_err() {
        assert_eq!(spoiler("||text"), Err(parse_error("", ErrorKind::Eof)));
        assert_eq!(
            spoiler("text||"),
            Err(parse_error("text||", ErrorKind::Tag))
        );
        assert_eq!(
            spoiler("|text|"),
            Err(parse_error("|text|", ErrorKind::Tag))
        );
        assert_eq!(spoiler("text"), Err(parse_error("text", ErrorKind::Tag)));
        assert_eq!(spoiler("||||"), Err(parse_error("||", ErrorKind::Verify)));
    }

    #[test]
    fn test_one_line_code_ok() {
        assert_eq!(
            one_line_code("`*text*`"),
            Ok(("", OneLineCode::new("*text*")))
        );
    }

    #[test]
    fn test_one_line_code_err() {
        assert_eq!(
            one_line_code("`*text*"),
            Err(parse_error("", ErrorKind::Eof))
        );
        assert_eq!(
            one_line_code("*text*`"),
            Err(parse_error("*text*`", ErrorKind::Tag))
        );
        assert_eq!(
            one_line_code("*text*"),
            Err(parse_error("*text*", ErrorKind::Tag))
        );
        assert_eq!(
            one_line_code("``"),
            Err(parse_error("`", ErrorKind::Verify))
        );
    }

    #[test]
    fn test_multi_line_code_ok() {
        assert_eq!(
            multi_line_code("```\nhello\nworld\n```"),
            Ok(("", MultiLineCode::new("\nhello\nworld\n", None)))
        );
        assert_eq!(
            multi_line_code("```hello world```"),
            Ok(("", MultiLineCode::new("hello world", None)))
        );
        assert_eq!(
            multi_line_code("``` hello\nworld```"),
            Ok(("", MultiLineCode::new(" hello\nworld", None)))
        );
        assert_eq!(
            multi_line_code("```\nhello\n```world"),
            Ok(("world", MultiLineCode::new("\nhello\n", None)))
        );
    }

    #[test]
    fn test_multi_line_code_err() {
        assert_eq!(
            multi_line_code("```hello"),
            Err(parse_error("", ErrorKind::Eof))
        );
        assert_eq!(
            multi_line_code("hello```"),
            Err(parse_error("hello```", ErrorKind::Tag))
        );
        assert_eq!(
            multi_line_code("hello"),
            Err(parse_error("hello", ErrorKind::Tag))
        );
        assert_eq!(
            multi_line_code("``````"),
            Err(parse_error("```", ErrorKind::Verify))
        );
    }

    #[test]
    fn test_multi_line_code_with_lang_ok() {
        assert_eq!(
            multi_line_code("```js\nhello\nworld\n```"),
            Ok((
                "",
                MultiLineCode::new("\nhello\nworld\n", Some("js".to_string()))
            ))
        );
        assert_eq!(
            multi_line_code("```x86asm\nhello```"),
            Ok((
                "",
                MultiLineCode::new("\nhello", Some("x86asm".to_string()))
            ))
        );
    }

    #[test]
    fn test_multi_line_code_with_lang_err() {
        assert_eq!(
            multi_line_code("```js\nhello"),
            Err(parse_error("", ErrorKind::Eof))
        );
    }
}
