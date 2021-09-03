//! Markdown AST structure
//!
//! [`ast`](crate::ast) module provides AST components.
//!
//! Note: [`builder`](crate::builder) module provides helper functions to build AST in less lines of code.
//!
//! # AST structure
//!
//! An AST consists of [`MarkdownDocument`], [`MarkdownElementCollection`], and [`MarkdownElement`].
//!
//! - [`MarkdownDocument`] is the root of AST. It contains a [`MarkdownElementCollection`].
//! - [`MarkdownElementCollection`] is a collection of markdown elements. It consists of one or more [`MarkdownElement`].
//! - [`MarkdownElement`] is a markdown element, such as `plain text` and `*italics text*`. The content of [`MarkdownElement`] can be nested. For instance, `__*nested* styles__` is also valid markdown element.
//!
//! # Generating markdown text
//!
//! `MarkdownDocument::to_string()` generates markdown text from the AST.
//!
//! ## Example
//!
//! ```
//! use discord_md::ast::*;
//!
//! let ast = MarkdownDocument::new(vec![
//!     MarkdownElement::Bold(Box::new(Bold::new("bold"))),
//!     MarkdownElement::Plain(Box::new(Plain::new(" text")))
//! ]);
//!
//! assert_eq!(ast.to_string(), "**bold** text");
//! ```

use derive_more::{Display, From, Into, IntoIterator};
use std::fmt;

/// A markdown document. The root of AST.
///
/// # Generating markdown text
///
/// `to_string()` generates markdown text from the AST.
///
/// ## Example
///
/// ```
/// use discord_md::ast::*;
///
/// let ast = MarkdownDocument::new(vec![
///     MarkdownElement::Bold(Box::new(Bold::new("bold text")))
/// ]);
///
/// assert_eq!(ast.to_string(), "**bold text**");
/// ```
#[derive(Debug, Eq, PartialEq, Display)]
pub struct MarkdownDocument {
    content: MarkdownElementCollection,
}

impl MarkdownDocument {
    /// Creates a markdown document.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of the markdown document.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of the markdown document as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// A collection of [`MarkdownElement`].
#[derive(Debug, Eq, PartialEq, From, Into, IntoIterator)]
pub struct MarkdownElementCollection(Vec<MarkdownElement>);

impl MarkdownElementCollection {
    /// Creates a collection of markdown element.
    pub fn new(value: Vec<MarkdownElement>) -> Self {
        Self(value)
    }

    /// Returns the collection of markdown element in [`Vec`].
    pub fn get(&self) -> &Vec<MarkdownElement> {
        &self.0
    }

    /// Returns the content of the collection as plain text.
    pub fn to_plain_string(&self) -> String {
        self.0
            .iter()
            .map(|c| c.to_plain_string())
            .collect::<String>()
    }
}

impl fmt::Display for MarkdownElementCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|c| c.to_string()).collect::<String>()
        )
    }
}

impl From<MarkdownElement> for MarkdownElementCollection {
    fn from(value: MarkdownElement) -> Self {
        MarkdownElementCollection::new(vec![value])
    }
}

impl From<&str> for MarkdownElementCollection {
    fn from(value: &str) -> Self {
        MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(value)))])
    }
}

impl From<String> for MarkdownElementCollection {
    fn from(value: String) -> Self {
        MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(value)))])
    }
}

impl From<&String> for MarkdownElementCollection {
    fn from(value: &String) -> Self {
        MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(value)))])
    }
}

/// A markdown element.
#[derive(Debug, Eq, PartialEq, Display)]
pub enum MarkdownElement {
    /// Plain text.
    Plain(Box<Plain>),

    /// Italics text, wrapped in `*`.
    ItalicsStar(Box<ItalicsStar>),

    /// Italics text, wrapped in `_`.
    ItalicsUnderscore(Box<ItalicsUnderscore>),

    /// Bold text, wrapped in `**`.
    Bold(Box<Bold>),

    /// Underline text, wrapped in `__`.
    Underline(Box<Underline>),

    /// Strikethrough text, wrapped in `~~`.
    Strikethrough(Box<Strikethrough>),

    /// Spoiler text, wrapped in `||`.
    Spoiler(Box<Spoiler>),

    /// Inline code block, wrapped in `` ` ``.
    OneLineCode(Box<OneLineCode>),

    /// Multiline code block, wrapped in ```` ``` ````.
    MultiLineCode(Box<MultiLineCode>),

    /// Block quote, preceded by `> `.
    BlockQuote(Box<BlockQuote>),
}

impl MarkdownElement {
    /// Returns the content of the element as plain text.
    pub fn to_plain_string(&self) -> String {
        match self {
            MarkdownElement::Plain(x) => x.content().to_string(),
            MarkdownElement::ItalicsStar(x) => x.to_plain_text(),
            MarkdownElement::ItalicsUnderscore(x) => x.to_plain_text(),
            MarkdownElement::Bold(x) => x.to_plain_text(),
            MarkdownElement::Underline(x) => x.to_plain_text(),
            MarkdownElement::Strikethrough(x) => x.to_plain_text(),
            MarkdownElement::Spoiler(x) => x.to_plain_text(),
            MarkdownElement::OneLineCode(x) => x.content().to_string(),
            MarkdownElement::MultiLineCode(x) => x.content().to_string(),
            MarkdownElement::BlockQuote(x) => x.to_plain_text(),
        }
    }
}

/// Plain text.
///
/// # Example markdown text
///
/// `plain text` (plain text)
#[derive(Debug, Eq, PartialEq, Display)]
pub struct Plain {
    content: String,
}

impl Plain {
    /// Creates plain text.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of plain text.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Italics text, wrapped in `*`.
///
/// # Example markdown text
///
/// `*italics text*` (*italics text*)
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "*{}*", content)]
pub struct ItalicsStar {
    content: MarkdownElementCollection,
}

impl ItalicsStar {
    /// Creates italics text wrapped in `*`.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of italics text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of italics text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// Italics text, wrapped in `_`.
///
/// # Example markdown text
///
/// `_italics text_` (_italics text_)
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "_{}_", content)]
pub struct ItalicsUnderscore {
    content: MarkdownElementCollection,
}

impl ItalicsUnderscore {
    /// Creates italics text wrapped in `_`.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of italics text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of italics text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// Bold text, wrapped in `**`.
///
/// # Example markdown text
///
/// `**bold text**` (**bold text**)
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "**{}**", content)]
pub struct Bold {
    content: MarkdownElementCollection,
}

impl Bold {
    /// Creates bold text.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of bold text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of bold text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// Underline text, wrapped in `__`.
///
/// # Example markdown text
///
/// `__underline text__`
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "__{}__", content)]
pub struct Underline {
    content: MarkdownElementCollection,
}

impl Underline {
    /// Creates underline text.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of underline text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of bold text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// Strikethrough text, wrapped in `~~`.
///
/// # Example markdown text
///
/// `~~strikethrough text~~` (~~strikethrough text~~)
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "~~{}~~", content)]
pub struct Strikethrough {
    content: MarkdownElementCollection,
}

impl Strikethrough {
    /// Creates strikethrough text.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of strikethrough text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of strikethrough text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// Spoiler text, wrapped in `||`.
///
/// # Example markdown text
///
/// `||spoiler text||`
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "||{}||", content)]
pub struct Spoiler {
    content: MarkdownElementCollection,
}

impl Spoiler {
    /// Creates spoiler text.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of spoiler text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of spoiler text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

/// Inline code block, wrapped in `` ` ``.
///
/// # Example markdown text
///
/// `` `let foo = "bar";` `` (`let foo = "bar";`)
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "`{}`", content)]
pub struct OneLineCode {
    content: String,
}

impl OneLineCode {
    /// Creates an inline code block.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of the code block.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Multiline code block, wrapped in ```` ``` ````.
///
/// # Example markdown text
///
/// ````text
/// ```html
/// <p>
///   code block
/// </p>
/// ```
/// ````
#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "```{}{}```", r#"language.as_deref().unwrap_or("")"#, content)]
pub struct MultiLineCode {
    content: String,
    language: Option<String>,
}

impl MultiLineCode {
    /// Creates a multiline code block.
    pub fn new(content: impl Into<String>, language: Option<String>) -> Self {
        // language の型を Option<impl Into<String>> にしたいが、そうすると None を渡せなくなる
        // never type の実装を待つ必要がありそう
        // https://stackoverflow.com/q/42141129
        Self {
            content: content.into(),
            language,
        }
    }

    /// Returns the content of the code block.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the language of the code block.
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
}

/// Block quote, preceded by `> `.
///
/// # Example markdown text
///
/// ```text
/// > this is
/// > block quote
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct BlockQuote {
    content: MarkdownElementCollection,
}

impl BlockQuote {
    /// Creates a block quote text.
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Returns the content of the block quote text.
    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }

    /// Returns the content of the block quote text as plain text.
    pub fn to_plain_text(&self) -> String {
        self.content.to_plain_string()
    }
}

impl fmt::Display for BlockQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .to_string()
                .split('\n')
                .map(|line| format!("> {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl From<Plain> for MarkdownElement {
    fn from(value: Plain) -> Self {
        MarkdownElement::Plain(Box::new(value))
    }
}

impl From<ItalicsStar> for MarkdownElement {
    fn from(value: ItalicsStar) -> Self {
        MarkdownElement::ItalicsStar(Box::new(value))
    }
}

impl From<ItalicsUnderscore> for MarkdownElement {
    fn from(value: ItalicsUnderscore) -> Self {
        MarkdownElement::ItalicsUnderscore(Box::new(value))
    }
}

impl From<Bold> for MarkdownElement {
    fn from(value: Bold) -> Self {
        MarkdownElement::Bold(Box::new(value))
    }
}

impl From<Underline> for MarkdownElement {
    fn from(value: Underline) -> Self {
        MarkdownElement::Underline(Box::new(value))
    }
}

impl From<Strikethrough> for MarkdownElement {
    fn from(value: Strikethrough) -> Self {
        MarkdownElement::Strikethrough(Box::new(value))
    }
}

impl From<Spoiler> for MarkdownElement {
    fn from(value: Spoiler) -> Self {
        MarkdownElement::Spoiler(Box::new(value))
    }
}

impl From<OneLineCode> for MarkdownElement {
    fn from(value: OneLineCode) -> Self {
        MarkdownElement::OneLineCode(Box::new(value))
    }
}

impl From<MultiLineCode> for MarkdownElement {
    fn from(value: MultiLineCode) -> Self {
        MarkdownElement::MultiLineCode(Box::new(value))
    }
}

impl From<BlockQuote> for MarkdownElement {
    fn from(value: BlockQuote) -> Self {
        MarkdownElement::BlockQuote(Box::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_content() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "plain",
            )))])
        };
        assert_eq!(MarkdownDocument::new(test_case()).content(), &test_case());
    }

    #[test]
    fn test_document_to_string() {
        assert_eq!(
            MarkdownDocument::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Bold(Box::new(Bold::new(MarkdownElementCollection::new(vec![
                    MarkdownElement::Plain(Box::new(Plain::new("bold"))),
                ])))),
                MarkdownElement::Plain(Box::new(Plain::new(" plain"))),
            ]))
            .to_string(),
            "**bold** plain"
        );
    }

    #[test]
    fn test_element_collection_get() {
        let test_case = || vec![MarkdownElement::Plain(Box::new(Plain::new("plain")))];
        assert_eq!(
            MarkdownElementCollection::new(test_case()).get(),
            &test_case()
        );
    }

    #[test]
    fn test_element_collection_to_string() {
        assert_eq!(
            MarkdownElementCollection::new(vec![
                MarkdownElement::Bold(Box::new(Bold::new(MarkdownElementCollection::new(vec![
                    MarkdownElement::Plain(Box::new(Plain::new("bold"))),
                ])))),
                MarkdownElement::Plain(Box::new(Plain::new(" plain "))),
                MarkdownElement::Underline(Box::new(Underline::new(
                    MarkdownElementCollection::new(vec![MarkdownElement::Bold(Box::new(
                        Bold::new(MarkdownElementCollection::new(vec![
                            MarkdownElement::Plain(Box::new(Plain::new("underline bold"))),
                        ]))
                    ))])
                ))),
            ])
            .to_string(),
            "**bold** plain __**underline bold**__"
        );
    }

    #[test]
    fn test_element_collection_from_element() {
        assert_eq!(
            MarkdownElementCollection::from(MarkdownElement::Plain(Box::new(Plain::new(
                "plain text"
            )))),
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "plain text"
            )))]),
        );
    }

    #[test]
    fn test_element_collection_from_str() {
        assert_eq!(
            MarkdownElementCollection::from("plain text"),
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "plain text"
            )))]),
        );
    }

    #[test]
    fn test_element_collection_from_string() {
        let test_case = "plain text".to_string();
        assert_eq!(
            MarkdownElementCollection::from(&test_case),
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "plain text"
            )))]),
        );
        assert_eq!(
            MarkdownElementCollection::from(test_case),
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "plain text"
            )))]),
        );
    }

    #[test]
    fn test_plain_content() {
        assert_eq!(Plain::new("plain text").content(), "plain text");
    }

    #[test]
    fn test_plain_to_string() {
        assert_eq!(Plain::new("plain text").to_string(), "plain text");
    }

    #[test]
    fn test_styled_content() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "text",
            )))])
        };

        assert_eq!(ItalicsStar::new(test_case()).content(), &test_case());
        assert_eq!(ItalicsUnderscore::new(test_case()).content(), &test_case());
        assert_eq!(Bold::new(test_case()).content(), &test_case());
        assert_eq!(Underline::new(test_case()).content(), &test_case());
        assert_eq!(Strikethrough::new(test_case()).content(), &test_case());
        assert_eq!(Spoiler::new(test_case()).content(), &test_case());
    }

    #[test]
    fn test_styled_to_string() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "text",
            )))])
        };

        assert_eq!(ItalicsStar::new(test_case()).to_string(), "*text*");
        assert_eq!(ItalicsUnderscore::new(test_case()).to_string(), "_text_");
        assert_eq!(Bold::new(test_case()).to_string(), "**text**");
        assert_eq!(Underline::new(test_case()).to_string(), "__text__");
        assert_eq!(Strikethrough::new(test_case()).to_string(), "~~text~~");
        assert_eq!(Spoiler::new(test_case()).to_string(), "||text||");
    }

    #[test]
    fn test_one_line_code_content() {
        assert_eq!(OneLineCode::new("one line code").content(), "one line code");
    }

    #[test]
    fn test_one_line_code_to_string() {
        assert_eq!(
            OneLineCode::new("one line code").to_string(),
            "`one line code`"
        );
    }

    #[test]
    fn test_multi_line_code_content() {
        assert_eq!(
            MultiLineCode::new("multi\nline\ncode\n", None).content(),
            "multi\nline\ncode\n"
        );
    }

    #[test]
    fn test_multi_line_code_language() {
        assert_eq!(
            MultiLineCode::new("multi\nline\ncode\n", Some("js".to_string())).language(),
            Some("js")
        );
        assert_eq!(
            MultiLineCode::new("multi\nline\ncode\n", None).language(),
            None
        );
    }

    #[test]
    fn test_multi_line_code_to_string() {
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", None).to_string(),
            "```\nmulti\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new(" multi\nline\ncode\n", None).to_string(),
            "``` multi\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new("multi line code", None).to_string(),
            "```multi line code```"
        );
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", Some("js".to_string())).to_string(),
            "```js\nmulti\nline\ncode\n```"
        );
    }

    #[test]
    fn test_block_quote_content() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(BlockQuote::new(test_case()).content(), &test_case());
    }

    #[test]
    fn test_block_quote_to_string() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(
            BlockQuote::new(test_case()).to_string(),
            "> block quote\n> text"
        );
    }

    #[test]
    fn test_element_from_plain() {
        assert_eq!(
            MarkdownElement::from(Plain::new("plain text")),
            MarkdownElement::Plain(Box::new(Plain::new("plain text")))
        );
    }

    #[test]
    fn test_element_from_styled() {
        let test_element_collection = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "text",
            )))])
        };

        assert_eq!(
            MarkdownElement::from(ItalicsStar::new(test_element_collection())),
            MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(ItalicsUnderscore::new(test_element_collection())),
            MarkdownElement::ItalicsUnderscore(Box::new(ItalicsUnderscore::new(
                test_element_collection()
            )))
        );
        assert_eq!(
            MarkdownElement::from(Bold::new(test_element_collection())),
            MarkdownElement::Bold(Box::new(Bold::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(Underline::new(test_element_collection())),
            MarkdownElement::Underline(Box::new(Underline::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(Strikethrough::new(test_element_collection())),
            MarkdownElement::Strikethrough(Box::new(Strikethrough::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(Spoiler::new(test_element_collection())),
            MarkdownElement::Spoiler(Box::new(Spoiler::new(test_element_collection())))
        );
    }

    #[test]
    fn test_element_from_one_line_code() {
        assert_eq!(
            MarkdownElement::from(OneLineCode::new("one line code")),
            MarkdownElement::OneLineCode(Box::new(OneLineCode::new("one line code")))
        );
    }

    #[test]
    fn test_element_from_multi_line_code() {
        assert_eq!(
            MarkdownElement::from(MultiLineCode::new(
                "multi\nline\ncode\n",
                Some("js".to_string())
            )),
            MarkdownElement::MultiLineCode(Box::new(MultiLineCode::new(
                "multi\nline\ncode\n",
                Some("js".to_string())
            )))
        );
    }

    #[test]
    fn test_element_from_block_quote() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(
            MarkdownElement::from(BlockQuote::new(test_case())),
            MarkdownElement::BlockQuote(Box::new(BlockQuote::new(test_case())))
        );
    }
}
