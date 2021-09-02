# discord-md
Parser and builder for Discord's subset of markdown, written in Rust

## Example

### Parser

```rust
use discord_md::ast::*;
use discord_md::parse;

fn main() {
  let message = "You can write *italics text*, `*inline code*`, and more!";

  assert_eq!(
    parse(message),
    MarkdownDocument::new(vec![
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
    ])
  );
}
```

## Features

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
- Block Quote (builder only)
  ```
  > block quote
  > some text
  ```

# Limitations

- Parser cannot parse block quote. `> ` are treated as plain text.
