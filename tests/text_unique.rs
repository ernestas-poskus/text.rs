extern crate text;
use text::Unique;
use text::Occurrence;

#[test]
fn test_unique() {
    let input: Vec<String> = (vec!["One", "One", "1", "1"]).iter().map(|w| w.to_string()).collect();

    let occurred = input.occurrence();
    let got = occurred.unique();

    assert_eq!(got.len(), 2);
}
