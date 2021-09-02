use discord_md::parse;

#[test]
fn test_parse_then_generate_1() {
    let message = include_str!("example.md");
    assert_eq!(parse(message).to_string(), message);
}

#[test]
fn test_parse_then_generate_2() {
    let message = include_str!("../README.md");
    assert_eq!(parse(message).to_string(), message);
}
