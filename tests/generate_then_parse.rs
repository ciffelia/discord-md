use discord_md::ast::MarkdownDocument;
use discord_md::builder::*;
use discord_md::generate::MarkdownToString;
use discord_md::parse;

#[test]
fn test_generate_then_parse() {
    let ast = MarkdownDocument::new(vec![
        italics_star(vec![
            plain("this "),
            italics_underscore("is"),
            plain(" "),
            underline(vec![plain("an "), strikethrough("example")]),
        ]),
        plain("\n"),
        bold(vec![one_line_code("mark\ndown")]),
        plain(" document.\n"),
        spoiler("Lorem ipsum ..."),
        plain("\n"),
        multi_line_code("\nsome\ncode", None),
    ]);

    assert_eq!(parse(&ast.to_markdown_string()), ast);
}
