extern crate text;
use text::Unique;

#[test]
fn test_unique() {
    let mut input: Vec<String> =
        (vec!["One", "One", "1", "1"]).iter().map(|w| w.to_string()).collect();
    assert_eq!(input.unique(), vec!["One", "1"]);
}
