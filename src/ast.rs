use derive_more::{Display, From, Into, IntoIterator};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Display)]
pub struct MarkdownDocument {
    content: MarkdownElementCollection,
}

impl MarkdownDocument {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

#[derive(Debug, Eq, PartialEq, From, Into, IntoIterator)]
pub struct MarkdownElementCollection(Vec<MarkdownElement>);

impl MarkdownElementCollection {
    pub fn new(value: Vec<MarkdownElement>) -> Self {
        Self(value)
    }

    pub fn get(&self) -> &Vec<MarkdownElement> {
        &self.0
    }
}

impl fmt::Display for MarkdownElementCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|c| c.to_string()).collect::<String>()
        )
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
pub enum MarkdownElement {
    Plain(Box<Plain>),
    ItalicsStar(Box<ItalicsStar>),
    ItalicsUnderscore(Box<ItalicsUnderscore>),
    Bold(Box<Bold>),
    Underline(Box<Underline>),
    Strikethrough(Box<Strikethrough>),
    Spoiler(Box<Spoiler>),
    OneLineCode(Box<OneLineCode>),
    MultiLineCode(Box<MultiLineCode>),
    BlockQuote(Box<BlockQuote>),
}

#[derive(Debug, Eq, PartialEq, Display)]
pub struct Plain {
    content: String,
}

impl Plain {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl From<Plain> for MarkdownElement {
    fn from(value: Plain) -> Self {
        MarkdownElement::Plain(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "*{}*", content)]
pub struct ItalicsStar {
    content: MarkdownElementCollection,
}

impl ItalicsStar {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl From<ItalicsStar> for MarkdownElement {
    fn from(value: ItalicsStar) -> Self {
        MarkdownElement::ItalicsStar(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "_{}_", content)]
pub struct ItalicsUnderscore {
    content: MarkdownElementCollection,
}

impl ItalicsUnderscore {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl From<ItalicsUnderscore> for MarkdownElement {
    fn from(value: ItalicsUnderscore) -> Self {
        MarkdownElement::ItalicsUnderscore(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "**{}**", content)]
pub struct Bold {
    content: MarkdownElementCollection,
}

impl Bold {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl From<Bold> for MarkdownElement {
    fn from(value: Bold) -> Self {
        MarkdownElement::Bold(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "__{}__", content)]
pub struct Underline {
    content: MarkdownElementCollection,
}

impl Underline {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl From<Underline> for MarkdownElement {
    fn from(value: Underline) -> Self {
        MarkdownElement::Underline(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "~~{}~~", content)]
pub struct Strikethrough {
    content: MarkdownElementCollection,
}

impl Strikethrough {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl From<Strikethrough> for MarkdownElement {
    fn from(value: Strikethrough) -> Self {
        MarkdownElement::Strikethrough(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "||{}||", content)]
pub struct Spoiler {
    content: MarkdownElementCollection,
}

impl Spoiler {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl From<Spoiler> for MarkdownElement {
    fn from(value: Spoiler) -> Self {
        MarkdownElement::Spoiler(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "`{}`", content)]
pub struct OneLineCode {
    content: String,
}

impl OneLineCode {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl From<OneLineCode> for MarkdownElement {
    fn from(value: OneLineCode) -> Self {
        MarkdownElement::OneLineCode(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq, Display)]
#[display(fmt = "```{}{}```", r#"language.as_deref().unwrap_or("")"#, content)]
pub struct MultiLineCode {
    content: String,
    language: Option<String>,
}

impl MultiLineCode {
    // 引数の型を Option<impl Into<String>> にすると、Noneを渡せない
    // never type の実装を待つ必要がありそう
    // https://stackoverflow.com/q/42141129
    pub fn new(content: impl Into<String>, language: Option<String>) -> Self {
        Self {
            content: content.into(),
            language,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
}

impl From<MultiLineCode> for MarkdownElement {
    fn from(value: MultiLineCode) -> Self {
        MarkdownElement::MultiLineCode(Box::new(value))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockQuote {
    content: MarkdownElementCollection,
}

impl BlockQuote {
    pub fn new(content: impl Into<MarkdownElementCollection>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &MarkdownElementCollection {
        &self.content
    }
}

impl fmt::Display for BlockQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .to_string()
                .split('\n')
                .map(|line| format!("> {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl From<BlockQuote> for MarkdownElement {
    fn from(value: BlockQuote) -> Self {
        MarkdownElement::BlockQuote(Box::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_content() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "plain",
            )))])
        };
        assert_eq!(MarkdownDocument::new(test_case()).content(), &test_case());
    }

    #[test]
    fn test_document_to_string() {
        assert_eq!(
            MarkdownDocument::new(MarkdownElementCollection::new(vec![
                Bold::new(vec![Plain::new("bold").into()]).into(),
                Plain::new(" plain").into()
            ]))
            .to_string(),
            "**bold** plain"
        );
    }

    #[test]
    fn test_element_collection_get() {
        let test_case = || vec![MarkdownElement::Plain(Box::new(Plain::new("plain")))];
        assert_eq!(
            MarkdownElementCollection::new(test_case()).get(),
            &test_case()
        );
    }

    #[test]
    fn test_element_collection_to_string() {
        assert_eq!(
            MarkdownElementCollection::new(vec![
                Bold::new(vec![Plain::new("bold").into()]).into(),
                Plain::new(" plain ").into(),
                Underline::new(vec![
                    Bold::new(vec![Plain::new("underline bold").into()]).into()
                ])
                .into(),
            ])
            .to_string(),
            "**bold** plain __**underline bold**__"
        );
    }

    #[test]
    fn test_plain_content() {
        assert_eq!(Plain::new("plain text").content(), "plain text");
    }

    #[test]
    fn test_plain_to_string() {
        assert_eq!(Plain::new("plain text").to_string(), "plain text");
    }

    #[test]
    fn test_element_from_plain() {
        assert_eq!(
            MarkdownElement::from(Plain::new("plain text")),
            MarkdownElement::Plain(Box::new(Plain::new("plain text")))
        );
    }

    #[test]
    fn test_styled_content() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "text",
            )))])
        };

        assert_eq!(ItalicsStar::new(test_case()).content(), &test_case());
        assert_eq!(ItalicsUnderscore::new(test_case()).content(), &test_case());
        assert_eq!(Bold::new(test_case()).content(), &test_case());
        assert_eq!(Underline::new(test_case()).content(), &test_case());
        assert_eq!(Strikethrough::new(test_case()).content(), &test_case());
        assert_eq!(Spoiler::new(test_case()).content(), &test_case());
    }

    #[test]
    fn test_styled_to_string() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "text",
            )))])
        };

        assert_eq!(ItalicsStar::new(test_case()).to_string(), "*text*");
        assert_eq!(ItalicsUnderscore::new(test_case()).to_string(), "_text_");
        assert_eq!(Bold::new(test_case()).to_string(), "**text**");
        assert_eq!(Underline::new(test_case()).to_string(), "__text__");
        assert_eq!(Strikethrough::new(test_case()).to_string(), "~~text~~");
        assert_eq!(Spoiler::new(test_case()).to_string(), "||text||");
    }

    #[test]
    fn test_element_from_styled() {
        let test_element_collection = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "text",
            )))])
        };

        assert_eq!(
            MarkdownElement::from(ItalicsStar::new(test_element_collection())),
            MarkdownElement::ItalicsStar(Box::new(ItalicsStar::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(ItalicsUnderscore::new(test_element_collection())),
            MarkdownElement::ItalicsUnderscore(Box::new(ItalicsUnderscore::new(
                test_element_collection()
            )))
        );
        assert_eq!(
            MarkdownElement::from(Bold::new(test_element_collection())),
            MarkdownElement::Bold(Box::new(Bold::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(Underline::new(test_element_collection())),
            MarkdownElement::Underline(Box::new(Underline::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(Strikethrough::new(test_element_collection())),
            MarkdownElement::Strikethrough(Box::new(Strikethrough::new(test_element_collection())))
        );
        assert_eq!(
            MarkdownElement::from(Spoiler::new(test_element_collection())),
            MarkdownElement::Spoiler(Box::new(Spoiler::new(test_element_collection())))
        );
    }

    #[test]
    fn test_one_line_code_content() {
        assert_eq!(OneLineCode::new("one line code").content(), "one line code");
    }

    #[test]
    fn test_one_line_code_to_string() {
        assert_eq!(
            OneLineCode::new("one line code").to_string(),
            "`one line code`"
        );
    }

    #[test]
    fn test_element_from_one_line_code() {
        assert_eq!(
            MarkdownElement::from(OneLineCode::new("one line code")),
            MarkdownElement::OneLineCode(Box::new(OneLineCode::new("one line code")))
        );
    }

    #[test]
    fn test_multi_line_code_content() {
        assert_eq!(
            MultiLineCode::new("multi\nline\ncode\n", None).content(),
            "multi\nline\ncode\n"
        );
    }

    #[test]
    fn test_multi_line_code_language() {
        assert_eq!(
            MultiLineCode::new("multi\nline\ncode\n", Some("js".to_string())).language(),
            Some("js")
        );
        assert_eq!(
            MultiLineCode::new("multi\nline\ncode\n", None).language(),
            None
        );
    }

    #[test]
    fn test_multi_line_code_to_string() {
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", None).to_string(),
            "```\nmulti\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new(" multi\nline\ncode\n", None).to_string(),
            "``` multi\nline\ncode\n```"
        );
        assert_eq!(
            MultiLineCode::new("multi line code", None).to_string(),
            "```multi line code```"
        );
        assert_eq!(
            MultiLineCode::new("\nmulti\nline\ncode\n", Some("js".to_string())).to_string(),
            "```js\nmulti\nline\ncode\n```"
        );
    }

    #[test]
    fn test_element_from_multi_line_code() {
        assert_eq!(
            MarkdownElement::from(MultiLineCode::new(
                "multi\nline\ncode\n",
                Some("js".to_string())
            )),
            MarkdownElement::MultiLineCode(Box::new(MultiLineCode::new(
                "multi\nline\ncode\n",
                Some("js".to_string())
            )))
        );
    }

    #[test]
    fn test_block_quote_content() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(BlockQuote::new(test_case()).content(), &test_case());
    }

    #[test]
    fn test_block_quote_to_string() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(
            BlockQuote::new(test_case()).to_string(),
            "> block quote\n> text"
        );
    }

    #[test]
    fn test_element_from_block_quote() {
        let test_case = || {
            MarkdownElementCollection::new(vec![MarkdownElement::Plain(Box::new(Plain::new(
                "block quote\ntext",
            )))])
        };

        assert_eq!(
            MarkdownElement::from(BlockQuote::new(test_case())),
            MarkdownElement::BlockQuote(Box::new(BlockQuote::new(test_case())))
        );
    }
}
