//! Helper functions to build AST in less lines of code.
//!
//! # Example
//!
//! ```
//! use discord_md::ast::MarkdownDocument;
//! use discord_md::builder::*;
//!
//! let ast = MarkdownDocument::new(vec![
//!     plain("generating "),
//!     one_line_code("markdown"),
//!     plain(" is "),
//!     underline(vec![
//!         bold("easy"),
//!         plain(" and "),
//!         bold("fun!"),
//!     ]),
//! ]);
//!
//! assert_eq!(
//!     ast.to_string(),
//!     "generating `markdown` is __**easy** and **fun!**__"
//! );
//! ```

use crate::ast::*;

/// Build plain text element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::plain;
///
/// let ast = MarkdownDocument::new(vec![
///     plain("plain text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "plain text"
/// );
/// ```
pub fn plain(content: impl Into<String>) -> MarkdownElement {
    MarkdownElement::Plain(Box::new(Plain::new(content)))
}

/// Build italics text element wrapped in `*`.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::italics_star;
///
/// let ast = MarkdownDocument::new(vec![
///     italics_star("italics text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "*italics text*"
/// );
/// ```
pub fn italics_star(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(content)))
}

/// Build italics text element wrapped in `_`.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::italics_underscore;
///
/// let ast = MarkdownDocument::new(vec![
///     italics_underscore("italics text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "_italics text_"
/// );
/// ```
pub fn italics_underscore(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::ItalicsUnderscore(Box::new(ItalicsUnderscore::new(content)))
}

/// Build bold text element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::bold;
///
/// let ast = MarkdownDocument::new(vec![
///     bold("bold text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "**bold text**"
/// );
/// ```
pub fn bold(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Bold(Box::new(Bold::new(content)))
}

/// Build underline text element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::underline;
///
/// let ast = MarkdownDocument::new(vec![
///     underline("underline text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "__underline text__"
/// );
/// ```
pub fn underline(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Underline(Box::new(Underline::new(content)))
}

/// Build strikethrough text element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::strikethrough;
///
/// let ast = MarkdownDocument::new(vec![
///     strikethrough("strikethrough text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "~~strikethrough text~~"
/// );
/// ```
pub fn strikethrough(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Strikethrough(Box::new(Strikethrough::new(content)))
}

/// Build spoiler text element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::spoiler;
///
/// let ast = MarkdownDocument::new(vec![
///     spoiler("spoiler text")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "||spoiler text||"
/// );
/// ```
pub fn spoiler(content: impl Into<MarkdownElementCollection>) -> MarkdownElement {
    MarkdownElement::Spoiler(Box::new(Spoiler::new(content)))
}

/// Build a inline code element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::one_line_code;
///
/// let ast = MarkdownDocument::new(vec![
///     one_line_code("some code")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "`some code`"
/// );
/// ```
pub fn one_line_code(content: impl Into<String>) -> MarkdownElement {
    MarkdownElement::OneLineCode(Box::new(OneLineCode::new(content)))
}

/// Build a multiline code block element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::multi_line_code;
///
/// let code = r#"
/// let foo = "bar";
/// "#;
///
/// let ast = MarkdownDocument::new(vec![
///     multi_line_code(code, Some("rust".to_string()))
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "```rust\nlet foo = \"bar\";\n```"
/// );
/// ```
pub fn multi_line_code(content: impl Into<String>, language: Option<String>) -> MarkdownElement {
    MarkdownElement::MultiLineCode(Box::new(MultiLineCode::new(content, language)))
}

/// Build block quote text element.
///
/// # Example
///
/// ```
/// use discord_md::ast::MarkdownDocument;
/// use discord_md::builder::block_quote;
///
/// let ast = MarkdownDocument::new(vec![
///     block_quote("block\nquote")
/// ]);
///
/// assert_eq!(
///     ast.to_string(),
///     "> block\n> quote"
/// );
/// ```
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
