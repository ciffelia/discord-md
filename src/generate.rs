//! Generates markdown text or plain text from an AST
//!
//! [`generate`](crate::generate) module provides [`ToMarkdownString`] trait, which provides methods
//! to generate markdown text or plain text from an AST.
//!
//! Note that every struct that implements [`ToMarkdownString`] also implements [`Display`](std::fmt::Display).
//! This means you can use [`to_string()`](std::string::ToString::to_string())
//! instead of [`to_markdown_string()`](`ToMarkdownString::to_markdown_string()).
//!
//! # Example
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::generate::{ToMarkdownString, ToMarkdownStringOption};
//!
//! let ast = MarkdownDocument::new(vec![
//!     MarkdownElement::Bold(Box::new(Bold::new("bold"))),
//!     MarkdownElement::Plain(Box::new(Plain::new(" text")))
//! ]);
//!
//! assert_eq!(ast.to_string(), "**bold** text");
//! assert_eq!(ast.to_markdown_string(&ToMarkdownStringOption::new()), "**bold** text");
//! assert_eq!(ast.to_markdown_string(&ToMarkdownStringOption::new().omit_format(true)), "bold text");
//! ```

use crate::ast::{
    BlockQuote, Bold, ItalicsStar, ItalicsUnderscore, MarkdownDocument, MarkdownElement,
    MarkdownElementCollection, MultiLineCode, OneLineCode, Plain, Spoiler, Strikethrough,
    Underline,
};

/// Struct that allows to alter [`to_markdown_string()`](`ToMarkdownString::to_markdown_string())'s behaviour.
/// # Example
///
/// ```
/// use discord_md::ast::*;
/// use discord_md::generate::{ToMarkdownString, ToMarkdownStringOption};
///
/// let ast = MarkdownDocument::new(vec![
///     MarkdownElement::Spoiler(Box::new(Spoiler::new("spoiler"))),
///     MarkdownElement::Plain(Box::new(Plain::new(" text")))
/// ]);
///
/// assert_eq!(ast.to_markdown_string(&ToMarkdownStringOption::new()), "||spoiler|| text");
/// assert_eq!(ast.to_markdown_string(&ToMarkdownStringOption::new().omit_format(true)), "spoiler text");
/// assert_eq!(ast.to_markdown_string(&ToMarkdownStringOption::new().omit_spoiler(true)), " text");
/// ```
#[derive(Default)]
#[non_exhaustive]
pub struct ToMarkdownStringOption {
    /// Omit markdown styling from the output
    pub omit_format: bool,

    /// Omit spoilers from the output
    pub omit_spoiler: bool,
}

impl ToMarkdownStringOption {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn omit_format(mut self, value: bool) -> Self {
        self.omit_format = value;
        self
    }

    pub fn omit_spoiler(mut self, value: bool) -> Self {
        self.omit_spoiler = value;
        self
    }
}

/// A trait for converting a markdown component into a String.
pub trait ToMarkdownString {
    /// Returns the content of the component as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String;
}

impl ToMarkdownString for MarkdownDocument {
    /// Returns the content of the document as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        self.content().to_markdown_string(option)
    }
}

impl ToMarkdownString for MarkdownElementCollection {
    /// Returns the content of the collection as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        self.get()
            .iter()
            .map(|c| c.to_markdown_string(option))
            .collect::<String>()
    }
}

impl ToMarkdownString for MarkdownElement {
    /// Returns the content of the element as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        match self {
            MarkdownElement::Plain(x) => x.to_markdown_string(option),
            MarkdownElement::ItalicsStar(x) => x.to_markdown_string(option),
            MarkdownElement::ItalicsUnderscore(x) => x.to_markdown_string(option),
            MarkdownElement::Bold(x) => x.to_markdown_string(option),
            MarkdownElement::Underline(x) => x.to_markdown_string(option),
            MarkdownElement::Strikethrough(x) => x.to_markdown_string(option),
            MarkdownElement::Spoiler(x) => x.to_markdown_string(option),
            MarkdownElement::OneLineCode(x) => x.to_markdown_string(option),
            MarkdownElement::MultiLineCode(x) => x.to_markdown_string(option),
            MarkdownElement::BlockQuote(x) => x.to_markdown_string(option),
        }
    }
}

impl ToMarkdownString for Plain {
    /// Returns the content of the plain text.
    fn to_markdown_string(&self, _option: &ToMarkdownStringOption) -> String {
        self.content().to_string()
    }
}

impl ToMarkdownString for ItalicsStar {
    /// Returns the content of italics text as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_format {
            content
        } else {
            format!("*{}*", content)
        }
    }
}

impl ToMarkdownString for ItalicsUnderscore {
    /// Returns the content of italics text as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_format {
            content
        } else {
            format!("_{}_", content)
        }
    }
}

impl ToMarkdownString for Bold {
    /// Returns the content of bold text as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_format {
            content
        } else {
            format!("**{}**", content)
        }
    }
}

impl ToMarkdownString for Underline {
    /// Returns the content of underline text as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_format {
            content
        } else {
            format!("__{}__", content)
        }
    }
}

impl ToMarkdownString for Strikethrough {
    /// Returns the content of strikethrough text as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_format {
            content
        } else {
            format!("~~{}~~", content)
        }
    }
}

impl ToMarkdownString for Spoiler {
    /// Returns the content of spoiler text as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_spoiler {
            "".to_string()
        } else if option.omit_format {
            content
        } else {
            format!("||{}||", content)
        }
    }
}

impl ToMarkdownString for OneLineCode {
    /// Returns the content of the inline code block as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_string();

        if option.omit_format {
            content
        } else {
            format!("`{}`", content)
        }
    }
}

impl ToMarkdownString for MultiLineCode {
    /// Returns the content of the multiline code block as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_string();

        if option.omit_format {
            content
        } else {
            format!("```{}{}```", self.language().unwrap_or(""), content)
        }
    }
}

impl ToMarkdownString for BlockQuote {
    /// Returns the content of the block quote as markdown styled text.
    fn to_markdown_string(&self, option: &ToMarkdownStringOption) -> String {
        let content = self.content().to_markdown_string(option);

        if option.omit_format {
            content
        } else {
            content
                .split('\n')
                .map(|line| format!("> {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_text() -> MarkdownElementCollection {
        MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new("text")))])
    }

    fn option_default() -> ToMarkdownStringOption {
        ToMarkdownStringOption::new()
    }

    fn option_omit_format() -> ToMarkdownStringOption {
        ToMarkdownStringOption::new().omit_format(true)
    }

    fn option_omit_spoiler() -> ToMarkdownStringOption {
        ToMarkdownStringOption::new().omit_spoiler(true)
    }

    fn option_omit_format_and_spoiler() -> ToMarkdownStringOption {
        ToMarkdownStringOption::new()
            .omit_format(true)
            .omit_spoiler(true)
    }

    #[test]
    fn test_document_to_string() {
        let ast = MarkdownDocument::new(MarkdownElementCollection::new(vec![
            MarkdownElement::Spoiler(Box::new(Spoiler::new(MarkdownElementCollection::new(
                vec![MarkdownElement::Plain(Box::new(Plain::new("spoiler")))],
            )))),
            MarkdownElement::Plain(Box::new(Plain::new(" plain"))),
        ]));

        assert_eq!(
            ast.to_markdown_string(&option_default()),
            "||spoiler|| plain"
        );
        assert_eq!(
            ast.to_markdown_string(&option_omit_format()),
            "spoiler plain"
        );
        assert_eq!(ast.to_markdown_string(&option_omit_spoiler()), " plain");
        assert_eq!(
            ast.to_markdown_string(&option_omit_format_and_spoiler()),
            " plain"
        );
    }

    #[test]
    fn test_element_collection_to_string() {
        let ast = MarkdownElementCollection::new(vec![
            MarkdownElement::Spoiler(Box::new(Spoiler::new(MarkdownElementCollection::new(
                vec![MarkdownElement::Plain(Box::new(Plain::new("spoiler")))],
            )))),
            MarkdownElement::Plain(Box::new(Plain::new(" plain "))),
            MarkdownElement::Underline(Box::new(Underline::new(MarkdownElementCollection::new(
                vec![MarkdownElement::Bold(Box::new(Bold::new(
                    MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(
                        Plain::new("underline bold"),
                    ))]),
                )))],
            )))),
        ]);

        assert_eq!(
            ast.to_markdown_string(&option_default()),
            "||spoiler|| plain __**underline bold**__"
        );
        assert_eq!(
            ast.to_markdown_string(&option_omit_format()),
            "spoiler plain underline bold"
        );
        assert_eq!(
            ast.to_markdown_string(&option_omit_spoiler()),
            " plain __**underline bold**__"
        );
        assert_eq!(
            ast.to_markdown_string(&option_omit_format_and_spoiler()),
            " plain underline bold"
        );
    }

    #[test]
    fn test_plain_to_string() {
        let ast = Plain::new("plain text");

        assert_eq!(ast.to_markdown_string(&option_default()), "plain text");
        assert_eq!(ast.to_markdown_string(&option_omit_format()), "plain text");
    }

    #[test]
    fn test_italics_star_to_string() {
        assert_eq!(
            ItalicsStar::new(example_text()).to_markdown_string(&option_default()),
            "*text*"
        );
        assert_eq!(
            ItalicsStar::new(example_text()).to_markdown_string(&option_omit_format()),
            "text"
        );
    }

    #[test]
    fn test_italics_underscore_to_string() {
        assert_eq!(
            ItalicsUnderscore::new(example_text()).to_markdown_string(&option_default()),
            "_text_"
        );
        assert_eq!(
            ItalicsUnderscore::new(example_text()).to_markdown_string(&option_omit_format()),
            "text"
        );
    }

    #[test]
    fn test_bold_to_string() {
        assert_eq!(
            Bold::new(example_text()).to_markdown_string(&option_default()),
            "**text**"
        );
        assert_eq!(
            Bold::new(example_text()).to_markdown_string(&option_omit_format()),
            "text"
        );
    }

    #[test]
    fn test_underline_to_string() {
        assert_eq!(
            Underline::new(example_text()).to_markdown_string(&option_default()),
            "__text__"
        );
        assert_eq!(
            Underline::new(example_text()).to_markdown_string(&option_omit_format()),
            "text"
        );
    }

    #[test]
    fn test_strikethrough_to_string() {
        assert_eq!(
            Strikethrough::new(example_text()).to_markdown_string(&option_default()),
            "~~text~~"
        );
        assert_eq!(
            Strikethrough::new(example_text()).to_markdown_string(&option_omit_format()),
            "text"
        );
    }

    #[test]
    fn test_spoiler_to_string() {
        assert_eq!(
            Spoiler::new(example_text()).to_markdown_string(&option_default()),
            "||text||"
        );
        assert_eq!(
            Spoiler::new(example_text()).to_markdown_string(&option_omit_format()),
            "text"
        );
        assert_eq!(
            Spoiler::new(example_text()).to_markdown_string(&option_omit_spoiler()),
            ""
        );
        assert_eq!(
            Spoiler::new(example_text()).to_markdown_string(&option_omit_format_and_spoiler()),
            ""
        );
    }

    #[test]
    fn test_one_line_code_to_string() {
        assert_eq!(
            OneLineCode::new("one line code").to_markdown_string(&option_default()),
            "`one line code`"
        );
        assert_eq!(
            OneLineCode::new("one line code").to_markdown_string(&option_omit_format()),
            "one line code"
        );
    }

    #[test]
    fn test_multi_line_code_to_string() {
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", None).to_markdown_string(&option_default()),
            "```\nmulti\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", None)
                .to_markdown_string(&option_omit_format()),
            "\nmulti\nline\ncode\n"
        );

        assert_eq!(
            MultiLineCode::new(" multi\nline\ncode\n", None).to_markdown_string(&option_default()),
            "``` multi\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new(" multi\nline\ncode\n", None)
                .to_markdown_string(&option_omit_format()),
            " multi\nline\ncode\n"
        );

        assert_eq!(
            MultiLineCode::new("multi line code", None).to_markdown_string(&option_default()),
            "```multi line code```"
        );
        assert_eq!(
            MultiLineCode::new("multi line code", None).to_markdown_string(&option_omit_format()),
            "multi line code"
        );

        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", Some("js".to_string()))
                .to_markdown_string(&option_default()),
            "```js\nmulti\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", Some("js".to_string()))
                .to_markdown_string(&option_omit_format()),
            "\nmulti\nline\ncode\n"
        );
    }

    #[test]
    fn test_block_quote_to_string() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(
            BlockQuote::new(test_case()).to_markdown_string(&option_default()),
            "> block quote\n> text"
        );
        assert_eq!(
            BlockQuote::new(test_case()).to_markdown_string(&option_omit_format()),
            "block quote\ntext"
        );
    }
}
