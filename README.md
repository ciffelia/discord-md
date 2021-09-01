# discord-md
Parser and builder for Discord's subset of markdown, written in Rust

## Example

### Parser

```rust
use discord_md::ast::*;
use discord_md::parse;

fn main() {
  let message = "You can write *italics text*, ||spoilers||, `*inline code*`, and more!";

  assert_eq!(
    parse(message_1),
    MarkdownDocument::new(vec![
      Plain::new("You can write ").into(),
      ItalicsStar::new(vec![Plain::new("italics text").into()]).into(),
      Plain::new(", ").into(),
      Spoiler::new(vec![Plain::new("spoilers").into()]).into(),
      Plain::new(", ").into(),
      OneLineCode::new("*inline code*").into(),
      Plain::new(", and more!").into(),
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

- Parser cannot parse block quote.
