//! Helper functions to build AST in less lines of code.

use crate::ast::*;

pub fn plain(content: impl Into<String>) -> MarkdownElement {
    MarkdownElement::Plain(Box::new(Plain::new(content)))
}

pub fn italics_star(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(content)))
}

pub fn italics_underscore(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::ItalicsUnderscore(Box::new(ItalicsUnderscore::new(content)))
}

pub fn bold(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Bold(Box::new(Bold::new(content)))
}

pub fn underline(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Underline(Box::new(Underline::new(content)))
}

pub fn strikethrough(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Strikethrough(Box::new(Strikethrough::new(content)))
}

pub fn spoiler(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Spoiler(Box::new(Spoiler::new(content)))
}

pub fn one_line_code(content: impl Into<String>) -> MarkdownElement {
    MarkdownElement::OneLineCode(Box::new(OneLineCode::new(content)))
}

pub fn multi_line_code(content: impl Into<String>, language: Option<String>) -> MarkdownElement {
    MarkdownElement::MultiLineCode(Box::new(MultiLineCode::new(content, language)))
}

pub fn block_quote(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::BlockQuote(Box::new(BlockQuote::new(content)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        assert_eq!(
            MarkdownDocument::new(vec![plain("hello "), italics_star(vec![plain("world")])]),
            MarkdownDocument::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Plain(Box::new(Plain::new("hello "))),
                MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(
                    MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(
                        Plain::new("world")
                    ))])
                )))
            ]))
        );
        assert_eq!(
            MarkdownDocument::new(plain("plain text")),
            MarkdownDocument::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Plain(Box::new(Plain::new("plain text")))
            ]))
        );
        assert_eq!(
            MarkdownDocument::new("plain text"),
            MarkdownDocument::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Plain(Box::new(Plain::new("plain text")))
            ]))
        );
    }

    #[test]
    fn test_plain() {
        assert_eq!(
            plain("plain"),
            MarkdownElement::Plain(Box::new(Plain::new("plain")))
        );
    }

    #[test]
    fn test_italics_star() {
        assert_eq!(
            italics_star(vec![plain("italics")]),
            MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(
                MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                    "italics"
                )))])
            )))
        );
    }

    #[test]
    fn test_italics_underscore() {
        assert_eq!(
            italics_underscore(vec![plain("italics")]),
            MarkdownElement::ItalicsUnderscore(Box::new(ItalicsUnderscore::new(
                MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                    "italics"
                )))])
            )))
        );
    }

    #[test]
    fn test_bold() {
        assert_eq!(
            bold(vec![plain("bold")]),
            MarkdownElement::Bold(Box::new(Bold::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Plain(Box::new(Plain::new("bold")))
            ]))))
        );
    }

    #[test]
    fn test_underline() {
        assert_eq!(
            underline(vec![plain("underline")]),
            MarkdownElement::Underline(Box::new(Underline::new(MarkdownElementCollection::new(
                vec![MarkdownElement::Plain(Box::new(Plain::new("underline")))]
            ))))
        );
    }

    #[test]
    fn test_strikethrough() {
        assert_eq!(
            strikethrough(vec![plain("strikethrough")]),
            MarkdownElement::Strikethrough(Box::new(Strikethrough::new(
                MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                    "strikethrough"
                )))])
            )))
        );
    }

    #[test]
    fn test_spoiler() {
        assert_eq!(
            spoiler(vec![plain("spoiler")]),
            MarkdownElement::Spoiler(Box::new(Spoiler::new(MarkdownElementCollection::new(
                vec![MarkdownElement::Plain(Box::new(Plain::new("spoiler")))]
            ))))
        );
    }

    #[test]
    fn test_one_line_code() {
        assert_eq!(
            one_line_code("inline code"),
            MarkdownElement::OneLineCode(Box::new(OneLineCode::new("inline code")))
        );
    }

    #[test]
    fn test_multi_line_code() {
        assert_eq!(
            multi_line_code("*hello* world", None),
            MarkdownElement::MultiLineCode(Box::new(MultiLineCode::new("*hello* world", None)))
        );
        assert_eq!(
            multi_line_code("*hello* world", Some("markdown".to_string())),
            MarkdownElement::MultiLineCode(Box::new(MultiLineCode::new(
                "*hello* world",
                Some("markdown".to_string())
            )))
        );
    }
}
