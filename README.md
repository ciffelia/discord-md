# discord-md

[![CI Status](https://github.com/ciffelia/discord-md/workflows/CI/badge.svg?branch=main)](https://github.com/ciffelia/discord-md/actions?query=workflow%3ACI+branch%3Amain)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

Parser and generator for Discord's markdown, written in Rust

## Example

### Parsing

```rust
use discord_md::ast::*;
use discord_md::parse;

fn main() {
  let message = "You can write *italics text*, `*inline code*`, and more!";

  let ast = MarkdownDocument::new(vec![
    MarkdownElement::Plain(Box::new(
      Plain::new("You can write ")
    )),
    MarkdownElement::ItalicsStar(Box::new(
      ItalicsStar::new(vec![
        MarkdownElement::Plain(Box::new(
          Plain::new("italics text")
        ))
      ])
    )),
    MarkdownElement::Plain(Box::new(
      Plain::new(", ")
    )),
    MarkdownElement::OneLineCode(Box::new(
      OneLineCode::new("*inline code*")
    )),
    MarkdownElement::Plain(Box::new(
      Plain::new(", and more!")
    )),
  ]);

  assert_eq!(
    parse(message),
    ast
  );
}
```

### Generating

```rust
use discord_md::ast::MarkdownDocument;
use discord_md::builder::*;

fn main() {
  let ast = MarkdownDocument::new(vec![
    plain("generating "),
    one_line_code("markdown"),
    plain(" is "),
    underline(vec![
      bold("easy"),
      plain(" and "),
      bold("fun!"),
    ]),
  ]);

  assert_eq!(
    ast.to_string(),
    "generating `markdown` is __**easy** and **fun!**__"
  );
}
```

## Features

- Tested
- Well documented
- Minimal dependencies (only [nom](https://github.com/Geal/nom) and [derive_more](https://github.com/JelteF/derive_more))
- Supports the following syntax:
  - Italics (`*italics*`, `_italics_`)
  - Bold (`**bold**`)
  - Underline (`__underline__`)
  - Strikethrough (`~~strikethrough~~`)
  - Spoiler (`||spoiler||`)
  - One line code (`` `one line code` ``)
  - Multi line code
    ````
    ```sh
    echo "multi line"
    echo "code"
    ```
    ````
  - Block Quote ([generator only](#parser-limitations))
    ```
    > block quote
    > some text
    ```

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
discord-md = { git = "https://github.com/ciffelia/discord-md" }
```

## Documentation

Coming soon

## Parser limitations

The parser tries to mimic the behavior of the official Discord client's markdown parser, but it's not perfect. 
The following is the list of known limitations.

- Block quotes are not parsed. `> ` will be treated as plain text.
- Nested emphasis, like `*italics **bold italics** italics*`, may not be parsed properly.
- Intraword emphasis may not be handled properly. The parser treats `foo_bar_baz` as emphasis, while Discord's parser does not.
- Escaping sequence will be treated as plain text.

## License

[MIT License](LICENSE)
