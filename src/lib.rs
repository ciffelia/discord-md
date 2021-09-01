//! `discord_md` is a Rust library that provides parser and builder for Discord's subset of markdown.
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! discord-md = "0.1.0"
//! ```
//!
//! # Parsing
//!
//! [`parse`] parses a markdown document and returns AST.
//!
//! ## Example
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::parse;
//!
//! let message = "You can write *italics text*, `*inline code*`, and more!";
//!
//! assert_eq!(
//!     parse(message),
//!     MarkdownDocument::new(vec![
//!         MarkdownElement::Plain(Box::new(
//!             Plain::new("You can write ")
//!         )),
//!         MarkdownElement::ItalicsStar(Box::new(
//!             ItalicsStar::new(vec![
//!                 MarkdownElement::Plain(Box::new(
//!                     Plain::new("italics text")
//!                 ))
//!             ])
//!         )),
//!         MarkdownElement::Plain(Box::new(
//!             Plain::new(", ")
//!         )),
//!         MarkdownElement::OneLineCode(Box::new(
//!             OneLineCode::new("*inline code*")
//!         )),
//!         MarkdownElement::Plain(Box::new(
//!             Plain::new(", and more!")
//!         )),
//!     ])
//! );
//! ```
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::parse;
//!
//! let message = "Of course __*nested* styles__ are supported!";
//!
//! assert_eq!(
//!     parse(message),
//!     MarkdownDocument::new(vec![
//!         MarkdownElement::Plain(Box::new(
//!             Plain::new("Of course ")
//!         )),
//!         MarkdownElement::Underline(Box::new(
//!             Underline::new(vec![
//!                 MarkdownElement::ItalicsStar(Box::new(
//!                     ItalicsStar::new(vec![
//!                         MarkdownElement::Plain(Box::new(
//!                             Plain::new("nested")
//!                         )),
//!                     ])
//!                 )),
//!                 MarkdownElement::Plain(Box::new(
//!                     Plain::new(" styles")
//!                 )),
//!             ])
//!         )),
//!         MarkdownElement::Plain(Box::new(
//!             Plain::new(" are supported!")
//!         )),
//!     ])
//! );
//! ```
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::parse;
//!
//! let message = r#"```sh
//! echo "Code block is _available_ too!"
//! ```"#;
//!
//! assert_eq!(
//!     parse(message),
//!     MarkdownDocument::new(vec![
//!         MarkdownElement::MultiLineCode(Box::new(
//!             MultiLineCode::new(
//!                 "\necho \"Code block is _available_ too!\"\n",
//!                 Some("sh".to_string())
//!             )
//!         ))
//!     ])
//! );
//! ```

pub mod ast;
mod parser;

use ast::MarkdownDocument;

/// Parses a markdown document and returns AST.
/// ## Example
///
/// ```
/// use discord_md::ast::*;
/// use discord_md::parse;
///
/// let message = "this **is** markdown.";
///
/// assert_eq!(
///     parse(message),
///     MarkdownDocument::new(vec![
///         MarkdownElement::Plain(Box::new(
///             Plain::new("this ")
///         )),
///         MarkdownElement::Bold(Box::new(
///             Bold::new(vec![
///                 MarkdownElement::Plain(Box::new(
///                     Plain::new("is")
///                 ))
///             ])
///         )),
///         MarkdownElement::Plain(Box::new(
///             Plain::new(" markdown.")
///         )),
///     ])
/// );
/// ```
pub fn parse(msg: &str) -> MarkdownDocument {
    // Since there are no invalid markdown document, parsing should never fails.
    let (rest, doc) = parser::markdown_document(msg).unwrap();

    // All input should be consumed.
    assert!(rest.is_empty());

    doc
}

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::*;

    #[test]
    fn test_parse_1() {
        let message = "*italics*, ||spoilers||, `*inline code*`";
        assert_eq!(
            parse(message),
            MarkdownDocument::new(vec![
                ItalicsStar::new(vec![Plain::new("italics").into()]).into(),
                Plain::new(", ").into(),
                Spoiler::new(vec![Plain::new("spoilers").into()]).into(),
                Plain::new(", ").into(),
                OneLineCode::new("*inline code*").into(),
            ])
        );
    }

    #[test]
    fn test_parse_2() {
        let message = "__*nested* styles__ supported";
        assert_eq!(
            parse(message),
            MarkdownDocument::new(vec![
                Underline::new(vec![
                    ItalicsStar::new(vec![Plain::new("nested").into()]).into(),
                    Plain::new(" styles").into()
                ])
                .into(),
                Plain::new(" supported").into(),
            ])
        );
    }

    #[test]
    fn test_parse_3() {
        let message = r#"
```js
const cond = a > b || c < d || e === f;
```
        "#
        .trim();
        assert_eq!(
            parse(message),
            MarkdownDocument::new(vec![MultiLineCode::new(
                "\nconst cond = a > b || c < d || e === f;\n",
                Some("js".to_string())
            )
            .into()])
        );
    }
}
