//! Generates markdown text or plain text from an AST
//!
//! [`generate`](crate::generate) module provides [`MarkdownToString`] trait, which provides methods
//! to generate markdown text or plain text from an AST.
//!
//! Note that every struct that implements [`MarkdownToString`] also implements [`Display`](std::fmt::Display).
//! This means you can use [`to_string()`](std::string::ToString::to_string())
//! instead of [`to_markdown_string()`](`MarkdownToString::to_markdown_string()) (those two are equivalent).
//!
//! # Example
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::generate::MarkdownToString;
//!
//! let ast = MarkdownDocument::new(vec![
//!     MarkdownElement::Bold(Box::new(Bold::new("bold"))),
//!     MarkdownElement::Plain(Box::new(Plain::new(" text")))
//! ]);
//!
//! assert_eq!(ast.to_string(), "**bold** text");
//! assert_eq!(ast.to_markdown_string(), "**bold** text");
//! assert_eq!(ast.to_plain_string(), "bold text");
//! ```

use crate::ast::{
    BlockQuote, Bold, ItalicsStar, ItalicsUnderscore, MarkdownDocument, MarkdownElement,
    MarkdownElementCollection, MultiLineCode, OneLineCode, Plain, Spoiler, Strikethrough,
    Underline,
};

/// A trait for converting a markdown component to a String.
pub trait MarkdownToString {
    /// Returns the content of the component as markdown styled text.
    fn to_markdown_string(&self) -> String;

    /// Returns the content of the component as plain text.
    fn to_plain_string(&self) -> String;
}

impl MarkdownToString for MarkdownDocument {
    /// Returns the content of the document as markdown styled text.
    fn to_markdown_string(&self) -> String {
        self.content().to_markdown_string()
    }

    /// Returns the content of the document as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for MarkdownElementCollection {
    /// Returns the content of the collection as markdown styled text.
    fn to_markdown_string(&self) -> String {
        self.get()
            .iter()
            .map(|c| c.to_markdown_string())
            .collect::<String>()
    }

    /// Returns the content of the collection as plain text.
    fn to_plain_string(&self) -> String {
        self.get()
            .iter()
            .map(|c| c.to_plain_string())
            .collect::<String>()
    }
}

impl MarkdownToString for MarkdownElement {
    /// Returns the content of the element as markdown styled text.
    fn to_markdown_string(&self) -> String {
        match self {
            MarkdownElement::Plain(x) => x.to_markdown_string(),
            MarkdownElement::ItalicsStar(x) => x.to_markdown_string(),
            MarkdownElement::ItalicsUnderscore(x) => x.to_markdown_string(),
            MarkdownElement::Bold(x) => x.to_markdown_string(),
            MarkdownElement::Underline(x) => x.to_markdown_string(),
            MarkdownElement::Strikethrough(x) => x.to_markdown_string(),
            MarkdownElement::Spoiler(x) => x.to_markdown_string(),
            MarkdownElement::OneLineCode(x) => x.to_markdown_string(),
            MarkdownElement::MultiLineCode(x) => x.to_markdown_string(),
            MarkdownElement::BlockQuote(x) => x.to_markdown_string(),
        }
    }

    /// Returns the content of the element as plain text.
    fn to_plain_string(&self) -> String {
        match self {
            MarkdownElement::Plain(x) => x.to_plain_string(),
            MarkdownElement::ItalicsStar(x) => x.to_plain_string(),
            MarkdownElement::ItalicsUnderscore(x) => x.to_plain_string(),
            MarkdownElement::Bold(x) => x.to_plain_string(),
            MarkdownElement::Underline(x) => x.to_plain_string(),
            MarkdownElement::Strikethrough(x) => x.to_plain_string(),
            MarkdownElement::Spoiler(x) => x.to_plain_string(),
            MarkdownElement::OneLineCode(x) => x.to_plain_string(),
            MarkdownElement::MultiLineCode(x) => x.to_plain_string(),
            MarkdownElement::BlockQuote(x) => x.to_plain_string(),
        }
    }
}

impl MarkdownToString for Plain {
    /// Returns the content of the plain text.
    fn to_markdown_string(&self) -> String {
        self.content().to_string()
    }

    /// Returns the content of the plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_string()
    }
}

impl MarkdownToString for ItalicsStar {
    /// Returns the content of italics text as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("*{}*", self.content().to_markdown_string())
    }

    /// Returns the content of italics text as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for ItalicsUnderscore {
    /// Returns the content of italics text as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("_{}_", self.content().to_markdown_string())
    }

    /// Returns the content of italics text as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for Bold {
    /// Returns the content of bold text as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("**{}**", self.content().to_markdown_string())
    }

    /// Returns the content of bold text as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for Underline {
    /// Returns the content of underline text as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("__{}__", self.content().to_markdown_string())
    }

    /// Returns the content of underline text as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for Strikethrough {
    /// Returns the content of strikethrough text as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("~~{}~~", self.content().to_markdown_string())
    }

    /// Returns the content of strikethrough text as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for Spoiler {
    /// Returns the content of spoiler text as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("||{}||", self.content().to_markdown_string())
    }

    /// Returns the content of spoiler text as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

impl MarkdownToString for OneLineCode {
    /// Returns the content of the inline code block as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!("`{}`", self.content())
    }

    /// Returns the content of the inline code block as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_string()
    }
}

impl MarkdownToString for MultiLineCode {
    /// Returns the content of the multiline code block as markdown styled text.
    fn to_markdown_string(&self) -> String {
        format!(
            "```{}{}```",
            self.language().as_deref().unwrap_or(""),
            self.content()
        )
    }

    /// Returns the content of the multiline code block as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_string()
    }
}

impl MarkdownToString for BlockQuote {
    /// Returns the content of the block quote as markdown styled text.
    fn to_markdown_string(&self) -> String {
        self.content()
            .to_markdown_string()
            .split('\n')
            .map(|line| format!("> {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns the content of the block quote as plain text.
    fn to_plain_string(&self) -> String {
        self.content().to_plain_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_text() -> MarkdownElementCollection {
        MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new("text")))])
    }

    #[test]
    fn test_document_to_string() {
        let ast = MarkdownDocument::new(MarkdownElementCollection::new(vec![
            MarkdownElement::Bold(Box::new(Bold::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Plain(Box::new(Plain::new("bold"))),
            ])))),
            MarkdownElement::Plain(Box::new(Plain::new(" plain"))),
        ]));

        assert_eq!(ast.to_markdown_string(), "**bold** plain");
        assert_eq!(ast.to_plain_string(), "bold plain");
    }

    #[test]
    fn test_element_collection_to_string() {
        let ast = MarkdownElementCollection::new(vec![
            MarkdownElement::Bold(Box::new(Bold::new(MarkdownElementCollection::new(vec![
                MarkdownElement::Plain(Box::new(Plain::new("bold"))),
            ])))),
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
            ast.to_markdown_string(),
            "**bold** plain __**underline bold**__"
        );
        assert_eq!(ast.to_plain_string(), "bold plain underline bold");
    }

    #[test]
    fn test_plain_to_string() {
        let ast = Plain::new("plain text");

        assert_eq!(ast.to_markdown_string(), "plain text");
        assert_eq!(ast.to_plain_string(), "plain text");
    }

    #[test]
    fn test_italics_star_to_string() {
        assert_eq!(
            ItalicsStar::new(example_text()).to_markdown_string(),
            "*text*"
        );
        assert_eq!(ItalicsStar::new(example_text()).to_plain_string(), "text");
    }

    #[test]
    fn test_italics_underscore_to_string() {
        assert_eq!(
            ItalicsUnderscore::new(example_text()).to_markdown_string(),
            "_text_"
        );
        assert_eq!(
            ItalicsUnderscore::new(example_text()).to_plain_string(),
            "text"
        );
    }

    #[test]
    fn test_bold_to_string() {
        assert_eq!(Bold::new(example_text()).to_markdown_string(), "**text**");
        assert_eq!(Bold::new(example_text()).to_plain_string(), "text");
    }

    #[test]
    fn test_underline_to_string() {
        assert_eq!(
            Underline::new(example_text()).to_markdown_string(),
            "__text__"
        );
        assert_eq!(Underline::new(example_text()).to_plain_string(), "text");
    }

    #[test]
    fn test_strikethrough_to_string() {
        assert_eq!(
            Strikethrough::new(example_text()).to_markdown_string(),
            "~~text~~"
        );
        assert_eq!(Strikethrough::new(example_text()).to_plain_string(), "text");
    }

    #[test]
    fn test_spoiler_to_string() {
        assert_eq!(
            Spoiler::new(example_text()).to_markdown_string(),
            "||text||"
        );
        assert_eq!(Spoiler::new(example_text()).to_plain_string(), "text");
    }

    #[test]
    fn test_one_line_code_to_string() {
        assert_eq!(
            OneLineCode::new("one line code").to_markdown_string(),
            "`one line code`"
        );
        assert_eq!(
            OneLineCode::new("one line code").to_plain_string(),
            "one line code"
        );
    }

    #[test]
    fn test_multi_line_code_to_string() {
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", None).to_markdown_string(),
            "```\nmulti\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", None).to_plain_string(),
            "\nmulti\nline\ncode\n"
        );

        assert_eq!(
            MultiLineCode::new(" multi\nline\ncode\n", None).to_markdown_string(),
            "``` multi\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new(" multi\nline\ncode\n", None).to_plain_string(),
            " multi\nline\ncode\n"
        );

        assert_eq!(
            MultiLineCode::new("multi line code", None).to_markdown_string(),
            "```multi line code```"
        );
        assert_eq!(
            MultiLineCode::new("multi line code", None).to_plain_string(),
            "multi line code"
        );

        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", Some("js".to_string()))
                .to_markdown_string(),
            "```js\nmulti\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", Some("js".to_string())).to_plain_string(),
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
            BlockQuote::new(test_case()).to_markdown_string(),
            "> block quote\n> text"
        );
        assert_eq!(
            BlockQuote::new(test_case()).to_plain_string(),
            "block quote\ntext"
        );
    }
}
