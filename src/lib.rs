pub mod ast;
mod parser;

use crate::ast::MarkdownDocument;

pub fn parse(msg: &str) -> MarkdownDocument {
    // Since there are no invalid markdown document, parsing should never fails.
    let (rest, doc) = parser::markdown_document(msg).unwrap();

    // All input should be consumed.
    assert!(rest.is_empty());

    doc
}
