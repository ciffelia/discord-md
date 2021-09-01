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
//! let message = "You can write *italics text*, ||spoilers||, `*inline code*`, and more!";
//! assert_eq!(
//!     parse(message),
//!     MarkdownDocument::new(vec![
//!         Plain::new("You can write ").into(),
//!         ItalicsStar::new(vec![Plain::new("italics text").into()]).into(),
//!         Plain::new(", ").into(),
//!         Spoiler::new(vec![Plain::new("spoilers").into()]).into(),
//!         Plain::new(", ").into(),
//!         OneLineCode::new("*inline code*").into(),
//!         Plain::new(", and more!").into(),
//!     ])
//! );
//! ```
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::parse;
//!
//! let message = "Of course __*nested* styles__ are supported!";
//! assert_eq!(
//!     parse(message),
//!     MarkdownDocument::new(vec![
//!         Plain::new("Of course ").into(),
//!         Underline::new(vec![
//!             ItalicsStar::new(vec![Plain::new("nested").into()]).into(),
//!             Plain::new(" styles").into()
//!         ])
//!         .into(),
//!         Plain::new(" are supported!").into(),
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
//! assert_eq!(
//!     parse(message),
//!     MarkdownDocument::new(vec![MultiLineCode::new(
//!         "\necho \"Code block is _available_ too!\"\n",
//!         Some("sh".to_string())
//!     )
//!     .into()])
//! );
//! ```

pub mod ast;
mod parser;

use crate::ast::MarkdownDocument;

/// Parses a markdown document and returns AST.
/// ## Example
///
/// ```
/// use discord_md::ast::*;
/// use discord_md::parse;
///
/// let message = "this **is** markdown.";
/// assert_eq!(
///     parse(message),
///     MarkdownDocument::new(vec![
///         Plain::new("this ").into(),
///         Bold::new(vec![Plain::new("is").into()]).into(),
///         Plain::new(" markdown.").into(),
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
