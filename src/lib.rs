//! discord-md is a Rust library that provides parser and builder for Discord's markdown.
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! discord-md = "2.0.0-rc.2"
//! ```
//!
//! # Parsing
//!
//! [`parse`] parses a markdown document and returns an AST.
//!
//! ## Example
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::parse;
//!
//! let message = "You can write *italics text*, `*inline code*`, and more!";
//!
//! let ast = MarkdownDocument::new(vec![
//!     MarkdownElement::Plain(Box::new(
//!         Plain::new("You can write ")
//!     )),
//!     MarkdownElement::ItalicsStar(Box::new(
//!         ItalicsStar::new(vec![
//!             MarkdownElement::Plain(Box::new(
//!                 Plain::new("italics text")
//!             ))
//!         ])
//!     )),
//!     MarkdownElement::Plain(Box::new(
//!         Plain::new(", ")
//!     )),
//!     MarkdownElement::OneLineCode(Box::new(
//!         OneLineCode::new("*inline code*")
//!     )),
//!     MarkdownElement::Plain(Box::new(
//!         Plain::new(", and more!")
//!     )),
//! ]);
//!
//! assert_eq!(
//!     parse(message),
//!     ast
//! );
//! ```
//!
//! ```
//! use discord_md::ast::*;
//! use discord_md::parse;
//!
//! let message = "Of course __*nested* styles__ are supported!";
//!
//! let ast = MarkdownDocument::new(vec![
//!     MarkdownElement::Plain(Box::new(
//!         Plain::new("Of course ")
//!     )),
//!     MarkdownElement::Underline(Box::new(
//!         Underline::new(vec![
//!             MarkdownElement::ItalicsStar(Box::new(
//!                 ItalicsStar::new(vec![
//!                     MarkdownElement::Plain(Box::new(
//!                         Plain::new("nested")
//!                     ))
//!                 ])
//!             )),
//!             MarkdownElement::Plain(Box::new(
//!                 Plain::new(" styles")
//!             )),
//!         ])
//!     )),
//!     MarkdownElement::Plain(Box::new(
//!         Plain::new(" are supported!")
//!     )),
//! ]);
//!
//! assert_eq!(
//!     parse(message),
//!     ast
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
//! let ast = MarkdownDocument::new(vec![
//!     MarkdownElement::MultiLineCode(Box::new(
//!         MultiLineCode::new(
//!             "\necho \"Code block is _available_ too!\"\n",
//!             Some("sh".to_string())
//!         )
//!     ))
//! ]);
//!
//! assert_eq!(
//!     parse(message),
//!     ast
//! );
//! ```
//!
//! # Generating
//!
//! First, build an AST with [`builder`] module.
//! Then call `to_string()` to generate markdown text from the AST.
//!
//! ## Example
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
//!
//! # Parser limitations
//!
//! The parser tries to mimic the behavior of the official Discord client's markdown parser, but it's not perfect.
//! The following is the list of known limitations.
//!
//! - Block quotes are not parsed. `> ` will be treated as plain text.
//! - Nested emphasis, like `*italics **bold italics** italics*`, may not be parsed properly.
//! - Intraword emphasis may not be handled properly. The parser treats `foo_bar_baz` as emphasis, while Discord's parser does not.
//! - Escaping sequence will be treated as plain text.

pub mod ast;
pub mod builder;
pub mod generate;
mod parser;

use ast::MarkdownDocument;

/// Parses a markdown document and returns AST.
///
/// # Example
///
/// ```
/// use discord_md::ast::*;
/// use discord_md::parse;
///
/// let message = "this **is** markdown.";
///
/// let ast = MarkdownDocument::new(vec![
///     MarkdownElement::Plain(Box::new(
///         Plain::new("this ")
///     )),
///     MarkdownElement::Bold(Box::new(
///         Bold::new(vec![
///             MarkdownElement::Plain(Box::new(
///                 Plain::new("is")
///             ))
///         ])
///     )),
///     MarkdownElement::Plain(Box::new(
///         Plain::new(" markdown.")
///     )),
/// ]);
///
/// assert_eq!(
///     parse(message),
///     ast
/// );
/// ```
///
/// # Limitations
///
/// The parser tries to mimic the behavior of the official Discord client's markdown parser, but it's not perfect.
/// The following is the list of known limitations.
///
/// - Block quotes are not parsed. `> ` will be treated as plain text.
/// - Nested emphasis, like `*italics **bold italics** italics*`, may not be parsed properly.
/// - Intraword emphasis may not be handled properly. The parser treats `foo_bar_baz` as emphasis, while Discord's parser does not.
/// - Escaping sequence will be treated as plain text.
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
                MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(vec![
                    MarkdownElement::Plain(Box::new(Plain::new("italics")))
                ]))),
                MarkdownElement::Plain(Box::new(Plain::new(", "))),
                MarkdownElement::Spoiler(Box::new(Spoiler::new(vec![MarkdownElement::Plain(
                    Box::new(Plain::new("spoilers"))
                )]))),
                MarkdownElement::Plain(Box::new(Plain::new(", "))),
                MarkdownElement::OneLineCode(Box::new(OneLineCode::new("*inline code*"))),
            ])
        );
    }

    #[test]
    fn test_parse_2() {
        let message = "__*nested* styles__ supported";
        assert_eq!(
            parse(message),
            MarkdownDocument::new(vec![
                MarkdownElement::Underline(Box::new(Underline::new(vec![
                    MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(vec![
                        MarkdownElement::Plain(Box::new(Plain::new("nested")))
                    ]))),
                    MarkdownElement::Plain(Box::new(Plain::new(" styles")))
                ]))),
                MarkdownElement::Plain(Box::new(Plain::new(" supported"))),
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
            MarkdownDocument::new(vec![MarkdownElement::MultiLineCode(Box::new(
                MultiLineCode::new(
                    "\nconst cond = a > b || c < d || e === f;\n",
                    Some("js".to_string())
                )
            ))])
        );
    }
}
