# discord-md

[![CI status][ci badge]][ci link]
[![crate version][crates.io badge]][crates.io link]
[![docs online][docs badge]][docs link]
[![MIT or Apache 2.0 Licenses][license badge]][license link]

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
discord-md = "2.0.0-rc.1"
```

## Documentation

[Available at docs.rs][docs link]

## Parser limitations

The parser tries to mimic the behavior of the official Discord client's markdown parser, but it's not perfect. 
The following is the list of known limitations.

- Block quotes are not parsed. `> ` will be treated as plain text.
- Nested emphasis, like `*italics **bold italics** italics*`, may not be parsed properly.
- Intraword emphasis may not be handled properly. The parser treats `foo_bar_baz` as emphasis, while Discord's parser does not.
- Escaping sequence will be treated as plain text.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[ci badge]: https://github.com/ciffelia/discord-md/workflows/CI/badge.svg?branch=main
[ci link]: https://github.com/ciffelia/discord-md/actions?query=workflow%3ACI+branch%3Amain

[crates.io badge]: https://img.shields.io/crates/v/discord-md
[crates.io link]: https://crates.io/crates/discord-md

[docs badge]: https://img.shields.io/badge/docs-online-green
[docs link]: https://docs.rs/discord-md

[license badge]: https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-blue
[license link]: #license
