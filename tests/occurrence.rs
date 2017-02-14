extern crate text;
use text::Occurrence;

#[test]
fn test_unique() {
    let input: Vec<String> =
        (vec!["One", "Single", "One", "1", "1"]).iter().map(|w| w.to_string()).collect();
    let got = input.occurrence();

    assert_eq!(got.get("One").unwrap(), &2);
    assert_eq!(got.get("1").unwrap(), &2);
    assert_eq!(got.get("Single").unwrap(), &1);
}
